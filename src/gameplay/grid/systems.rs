use bevy::prelude::{
    info, warn, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, Query, Res,
    ResMut, Transform, Vec2, Vec3, With,
};
use bevy_pkv::PkvStore;
use bevy_rapier3d::prelude::{CollisionEvent, RapierContext};
use hexx::{shapes, Hex};

use crate::{
    game_audio::utils::pkv_play_score_audio,
    gameplay::{
        ball::{
            components::{GridBall, OutBall, ProjectileBall, Species},
            events::SnapProjectile,
            grid_ball_bundle::GridBallBundle,
            out_ball_bundle::OutBallBundle,
            utils::clamp_inside_world_bounds,
        },
        constants::{MAX_COLS, MIN_CLUSTER_SIZE, MIN_COLS},
        events::BeginTurn,
        grid::utils::{find_cluster, find_floating_clusters},
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        panels::resources::{MoveCounter, ScoreCounter, TurnCounter},
    },
    loading::audio_assets::AudioAssets,
    resources::LevelCounter,
};

use super::{events::UpdatePositions, resources::Grid, utils::adjust_grid_layout};

pub fn generate_grid(
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut grid: ResMut<Grid>,
    mut update_positions: EventWriter<UpdatePositions>,
    level_counter: Res<LevelCounter>,
) {
    let factor: i32 = (level_counter.0 * 2) as i32;
    grid.init_cols = factor.clamp(MIN_COLS, MAX_COLS);
    grid.init_rows = factor;
    for hex in shapes::pointy_rectangle([0, grid.init_cols - 1, 0, grid.init_rows - 1]) {
        let (x, z) = grid.layout.hex_to_world_pos(hex).into();

        let entity = commands
            .spawn(GridBallBundle::new(
                Vec3::new(x, 0.0, z),
                grid.layout.hex_size.x,
                Species::random_species(),
                &gameplay_meshes,
                &gameplay_materials,
                hex,
            ))
            .id();

        grid.set(hex, entity);
    }

    // Center grid on x-axis.
    grid.update_bounds();
    let (width, _) = grid.dim();
    grid.layout.origin.x = -width / 2. + grid.layout.hex_size.x;
    adjust_grid_layout(&mut grid, &MoveCounter(0));
    update_positions.send(UpdatePositions);
}

pub const VISIBLE_ROWS: f32 = 5.0;

pub fn update_hex_coord_transforms(
    mut balls_query: Query<(Entity, &mut Transform, &GridBall), With<GridBall>>,
    mut grid: ResMut<Grid>,
    mut event_query: EventReader<UpdatePositions>,
    move_counter: Res<MoveCounter>,
) {
    if event_query.is_empty() {
        return;
    }
    event_query.clear();

    adjust_grid_layout(&mut grid, &move_counter);
    grid.update_bounds();

    for (entity, mut transform, GridBall { hex }) in balls_query.iter_mut() {
        let hex = *hex;
        grid.set(hex, entity);
        let (x, z) = grid.layout.hex_to_world_pos(hex).into();
        transform.translation.x = x;
        transform.translation.z = z;
    }
}

pub fn cleanup_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    grid_balls_query: Query<Entity, With<GridBall>>,
    out_balls_query: Query<Entity, With<OutBall>>,
) {
    for entity in grid_balls_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in out_balls_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    grid.clear();
}

pub fn check_projectile_out_of_grid(
    mut commands: Commands,
    mut projectile_query: Query<
        (Entity, &Transform, &ProjectileBall, &Species),
        With<ProjectileBall>,
    >,
    mut grid: ResMut<Grid>,
    mut snap_projectile: EventWriter<SnapProjectile>,
) {
    if let Ok((projectile_entity, projectile_transform, projectile_ball, species)) =
        projectile_query.get_single_mut()
    {
        if !projectile_ball.is_flying {
            return;
        }
        if grid.bounds.dirty {
            grid.update_bounds();
        }
        if projectile_transform.translation.z < grid.bounds.mins.y + grid.layout.hex_size.y {
            commands.entity(projectile_entity).despawn_recursive();
            snap_projectile.send(SnapProjectile {
                out_of_bounds: true,
                pos: Vec2::new(
                    projectile_transform.translation.x,
                    projectile_transform.translation.z,
                ),
                species: *species,
            });
        }
    }
}

pub fn on_projectile_collisions_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut snap_projectile: EventWriter<SnapProjectile>,
    mut projectile_query: Query<(Entity, &Transform, &Species), With<ProjectileBall>>,
    balls_query: Query<(Entity, &Transform), With<GridBall>>,
    rapier_context: Res<RapierContext>,
) {
    for (entity_a, entity_b) in collision_events.iter().filter_map(|e| match e {
        CollisionEvent::Started(a, b, _) => Some((a, b)),
        CollisionEvent::Stopped(_, _, _) => None,
    }) {
        if let Ok((ball_entity, ball_transform)) =
            balls_query.get(*entity_a).or(balls_query.get(*entity_b))
        {
            let mut p1 = projectile_query.get_mut(*entity_a);
            if p1.is_err() {
                p1 = projectile_query.get_mut(*entity_b);
            }
            println!(
                "ball_entity {} collision {} {}",
                ball_entity.index(),
                (*entity_a).index(),
                (*entity_b).index()
            );

            if let Some(contact_pair) = rapier_context.contact_pair(*entity_a, *entity_b) {
                if contact_pair.has_any_active_contacts() {
                    println!("manifolds_len {:?}", contact_pair.manifolds_len());
                }
            }

            if let Ok((projectile_entity, projectile_transform, species)) = p1 {
                println!(
                    "bt {:?} pt {:?}",
                    ball_transform.translation, projectile_transform.translation
                );
                // TODO set some flag to prevent despawn the same projectile
                // projectile can collide with two balls, use only first event
                commands.entity(projectile_entity).despawn_recursive();
                println!("SnapProjectile event");
                snap_projectile.send(SnapProjectile {
                    out_of_bounds: false,
                    pos: Vec2::new(
                        projectile_transform.translation.x,
                        projectile_transform.translation.z,
                    ),
                    species: *species,
                });
            }
        }
    }
}

pub fn on_snap_projectile(
    mut snap_projectile_events: EventReader<SnapProjectile>,
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut grid: ResMut<Grid>,
    mut begin_turn: EventWriter<BeginTurn>,
    mut score_counter: ResMut<ScoreCounter>,
    mut turn_counter: ResMut<TurnCounter>,
    mut move_counter: ResMut<MoveCounter>,
    balls_query: Query<(&Transform, &Species, &mut GridBall), With<GridBall>>,
    audio_assets: Res<AudioAssets>,
    pkv: Res<PkvStore>,
) {
    if let Some(snap_projectile) = snap_projectile_events.iter().next() {
        if grid.bounds.dirty {
            grid.update_bounds();
        }

        let mut hex = grid.layout.world_pos_to_hex(snap_projectile.pos);
        info!(
            "snap hex({}, {}) pos({}, {})",
            hex.x, hex.y, snap_projectile.pos.x, snap_projectile.pos.y
        );

        if !snap_projectile.out_of_bounds {
            // check to make sure the projectile is inside the grid bounds.
            let clamped = clamp_inside_world_bounds(
                snap_projectile.pos,
                grid.bounds.init_left,
                grid.bounds.init_right,
                grid.bounds.mins.y,
            );
            hex = grid.layout.world_pos_to_hex(clamped);
            info!("was_clamped hex({}, {}) {}", hex.x, hex.y, grid.bounds);

            let mut empty_neighbors = grid
                .empty_neighbors(hex)
                .iter()
                .filter_map(|e_hex| {
                    // get empty neighbors (free grid places)
                    // filter by min and max column (do not overflow left and right column)
                    // filter only that have neighbours in grid
                    match e_hex.x >= 0
                        && e_hex.x <= grid.init_cols
                        && grid.neighbors(*e_hex).len() > 0
                    {
                        true => Some(*e_hex),
                        false => None,
                    }
                })
                .collect::<Vec<Hex>>();
            info!("empty_neighbors {:?}", empty_neighbors);
            let mut grid_hex_option = grid.get(hex);
            while grid_hex_option.is_some() && empty_neighbors.len() > 0 {
                // this postition is already occupied in grid
                info!("found the same position in grid  hex({}, {})", hex.x, hex.y);
                if let Some(pop_hex) = empty_neighbors.pop() {
                    hex = pop_hex;
                    grid_hex_option = grid.get(hex);
                }
            }
            if grid_hex_option.is_some() {
                warn!("Can not snap projectile to grid, all possible places occupied!");
                return;
            }
        }

        let (x, z) = grid.layout.hex_to_world_pos(hex).into();
        let ball = commands
            .spawn(GridBallBundle::new(
                Vec3::new(x, 0.0, z),
                grid.layout.hex_size.x,
                snap_projectile.species,
                &gameplay_meshes,
                &gameplay_materials,
                hex,
            ))
            .id();

        grid.set(hex, ball); // add snapped projectile ball as grid ball
        grid.print_sorted_position();

        let mut score_add = 0;

        let neighbors = grid.neighbors(hex);
        if neighbors.len() == 0 {
            // projectile ball snapped with no neghbours
            // do not calc floating clusters
        } else {
            let (cluster, _) = find_cluster(&grid, hex, |&e| {
                e == ball
                    || match balls_query.get(e) {
                        Ok((_, other, _)) => *other == snap_projectile.species,
                        Err(_) => false,
                    }
            });

            // remove matching clusters
            if cluster.len() >= MIN_CLUSTER_SIZE {
                cluster.iter().for_each(|&c_hex| {
                    if let Some(grid_ball_entity) = grid.get(c_hex) {
                        if let Ok((ball_transform, ball_species, _)) =
                            balls_query.get(*grid_ball_entity)
                        {
                            commands.spawn(OutBallBundle::new(
                                ball_transform.translation,
                                *ball_species,
                                &gameplay_meshes,
                                &gameplay_materials,
                            ));
                        } else if c_hex.x == hex.x && c_hex.y == hex.y {
                            commands.spawn(OutBallBundle::new(
                                Vec3::new(x, 0.0, z),
                                snap_projectile.species,
                                &gameplay_meshes,
                                &gameplay_materials,
                            ));
                        }
                        commands.entity(*grid_ball_entity).despawn_recursive();
                    }
                    grid.remove(&c_hex);
                    score_add += 1;
                });
            }

            // remove floating clusters
            let floating_clusters = find_floating_clusters(&grid);
            floating_clusters
                .iter()
                .flat_map(|e| e.iter())
                .for_each(|&c_hex| {
                    if let Some(grid_ball_entity) = grid.get(c_hex) {
                        if let Ok((ball_transform, ball_species, _)) =
                            balls_query.get(*grid_ball_entity)
                        {
                            commands.spawn(OutBallBundle::new(
                                ball_transform.translation,
                                *ball_species,
                                &gameplay_meshes,
                                &gameplay_materials,
                            ));
                        } else if c_hex.x == hex.x && c_hex.y == hex.y {
                            commands.spawn(OutBallBundle::new(
                                Vec3::new(x, 0.0, z),
                                snap_projectile.species,
                                &gameplay_meshes,
                                &gameplay_materials,
                            ));
                        }
                        commands.entity(*grid_ball_entity).despawn_recursive();
                    }
                    grid.remove(&c_hex);
                    score_add += 1;
                });

            // grid.clear();

            if score_add > 0 {
                pkv_play_score_audio(&mut commands, &audio_assets, &pkv);
            }

            score_counter.0 += score_add;
        }

        turn_counter.0 += 1;
        if score_add == 0 {
            move_counter.0 += 1;
        }

        begin_turn.send(BeginTurn);
    }
    snap_projectile_events.clear();
}
