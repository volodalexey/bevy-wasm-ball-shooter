use bevy::{
    prelude::{
        error, info, Assets, BuildChildren, Commands, DespawnRecursiveExt, Entity, EventReader,
        EventWriter, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::ColorMaterial,
    time::Time,
};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::{CollisionEvent, RigidBody, Velocity};
use hexx::{shapes, Hex};

use crate::{
    game_audio::utils::pkv_play_score_audio,
    gameplay::{
        ball::{
            components::{GridBall, OutBall, ProjectileBall, Species},
            events::SnapProjectile,
            grid_ball_bundle::GridBallBundle,
            out_ball_bundle::OutBallBundle,
        },
        constants::MIN_CLUSTER_SIZE,
        events::BeginTurn,
        grid::utils::{
            clamp_inside_world_bounds, find_cluster, find_floating_clusters, is_move_slow,
        },
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        panels::resources::{CooldownMoveCounter, MoveCounter, ScoreCounter, TurnCounter},
        utils::calc_init_cols_rows,
    },
    loading::audio_assets::AudioAssets,
    resources::LevelCounter,
    utils::{from_2d_to_grid_2d, from_grid_2d_to_2d},
};

use super::{
    events::UpdatePositions,
    resources::{CollisionSnapCooldown, Grid},
    utils::{adjust_grid_layout, build_joints},
};

pub fn generate_grid(
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut grid: ResMut<Grid>,
    mut update_positions: EventWriter<UpdatePositions>,
    level_counter: Res<LevelCounter>,
) {
    (grid.init_cols, grid.init_rows) = calc_init_cols_rows(&level_counter);
    for hex in shapes::pointy_rectangle([0, grid.init_cols - 1, 0, grid.init_rows - 1]) {
        let hex_pos = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(hex));
        let is_first = hex.y == 0;

        let grid_ball_bundle = GridBallBundle::new(
            hex_pos,
            grid.layout.hex_size.x,
            Species::random_species(),
            &gameplay_meshes,
            &gameplay_materials,
            hex,
            match is_first {
                true => RigidBody::KinematicPositionBased,
                false => RigidBody::Dynamic,
            },
        );

        if is_first {
            let entity = commands.spawn(grid_ball_bundle).id();

            grid.set(hex, entity);
            continue;
        }

        let entity = commands
            .spawn(grid_ball_bundle)
            .with_children(|parent| {
                for joint in build_joints(hex, &grid) {
                    parent.spawn(joint);
                }
            })
            .id();
        grid.set(hex, entity);
    }

    // Center grid on x-axis.
    grid.check_update_bounds();
    let (width, _) = grid.dim();
    grid.layout.origin.x = -width / 2. + grid.layout.hex_size.x;
    adjust_grid_layout(&mut grid, &MoveCounter(0));
    grid.update_bounds();
    update_positions.send(UpdatePositions);
}

pub fn update_hex_coord_transforms(
    mut balls_query: Query<(Entity, &mut Transform, &mut Velocity, &GridBall), With<GridBall>>,
    mut grid: ResMut<Grid>,
    mut event_query: EventReader<UpdatePositions>,
    move_counter: Res<MoveCounter>,
) {
    if event_query.is_empty() {
        return;
    }
    event_query.clear();

    adjust_grid_layout(&mut grid, &move_counter);
    grid.check_update_bounds();

    for (entity, mut transform, mut velocity, GridBall { hex }) in balls_query.iter_mut() {
        let hex = *hex;
        grid.set(hex, entity);
        let pos_2d = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(hex));
        // println!("pos_2d {} {}", pos_2d.x, pos_2d.y);
        transform.translation.x = pos_2d.x;
        transform.translation.y = pos_2d.y;
        *velocity = Velocity::default();
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
        (Entity, &Transform, &mut ProjectileBall, &Species),
        With<ProjectileBall>,
    >,
    mut grid: ResMut<Grid>,
    mut snap_projectile: EventWriter<SnapProjectile>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
) {
    if let Ok((projectile_entity, projectile_transform, mut projectile_ball, species)) =
        projectile_query.get_single_mut()
    {
        if !projectile_ball.is_flying {
            return;
        }
        grid.check_update_bounds();
        if projectile_transform.translation.y > grid.bounds.maxs.y - grid.layout.hex_size.y {
            projectile_ball.is_ready_to_despawn = true;
            commands.entity(projectile_entity).despawn_recursive();
            info!(
                "Projectile out of grid snap {} {}",
                grid.bounds.mins.y, projectile_transform.translation.y
            );
            collision_snap_cooldown.stop();
            snap_projectile.send(SnapProjectile {
                pos: Vec2::new(
                    projectile_transform.translation.x,
                    projectile_transform.translation.y,
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
    mut projectile_query: Query<
        (Entity, &Transform, &Species, &Velocity, &mut ProjectileBall),
        With<ProjectileBall>,
    >,
    balls_query: Query<(Entity, &Transform), With<GridBall>>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
) {
    for (entity_a, entity_b, started) in collision_events.iter().map(|e| match e {
        CollisionEvent::Started(a, b, _) => (a, b, true),
        CollisionEvent::Stopped(a, b, _) => (a, b, false),
    }) {
        if let Ok((_, _)) = balls_query.get(*entity_a).or(balls_query.get(*entity_b)) {
            let mut p1 = projectile_query.get_mut(*entity_a);
            if p1.is_err() {
                p1 = projectile_query.get_mut(*entity_b);
            }

            if let Ok((
                projectile_entity,
                projectile_transform,
                species,
                velocity,
                mut projectile_ball,
            )) = p1
            {
                // take into account only collision between projectile and grid ball
                // println!("velocity {:?} len {}", velocity, velocity.linvel.length());
                if !projectile_ball.is_ready_to_despawn
                    && match started {
                        true => {
                            collision_snap_cooldown.start();
                            false
                        }
                        false => {
                            let is_slow = is_move_slow(velocity.linvel);
                            if is_slow {
                                collision_snap_cooldown.stop();
                            } else {
                                collision_snap_cooldown.start();
                            }
                            is_slow
                        }
                    }
                {
                    projectile_ball.is_ready_to_despawn = true;
                    // if ball turned back
                    // or ball moves too slow
                    commands.entity(projectile_entity).despawn_recursive();
                    info!("Projectile too slow so snap");
                    snap_projectile.send(SnapProjectile {
                        pos: Vec2::new(
                            projectile_transform.translation.x,
                            projectile_transform.translation.y,
                        ),
                        species: *species,
                    });
                }
            }
        }
    }
}

pub fn on_snap_projectile(
    mut snap_projectile_events: EventReader<SnapProjectile>,
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<Grid>,
    mut begin_turn: EventWriter<BeginTurn>,
    mut score_counter: ResMut<ScoreCounter>,
    mut turn_counter: ResMut<TurnCounter>,
    mut move_counter: ResMut<MoveCounter>,
    mut cooldown_move_counter: ResMut<CooldownMoveCounter>,
    balls_query: Query<(&Transform, &Species, &mut GridBall), With<GridBall>>,
    audio_assets: Res<AudioAssets>,
    pkv: Res<PkvStore>,
) {
    if let Some(snap_projectile) = snap_projectile_events.iter().next() {
        grid.check_update_bounds();

        // println!("{}", grid.bounds);
        let projectile_position = snap_projectile.pos;
        // let projectile_position = Vec2::new(-1.0528764, 12.633377);
        let mut hex = grid
            .layout
            .world_pos_to_hex(from_2d_to_grid_2d(projectile_position));
        // info!(
        //     "snap hex({}, {}) pos({}, {})",
        //     hex.x, hex.y, projectile_position.x, projectile_position.y
        // );
        // check to make sure the projectile is inside the grid bounds.
        (hex, _) = clamp_inside_world_bounds(&hex, &grid);
        info!("was_clamped by bounds hex({}, {})", hex.x, hex.y);

        let mut empty_neighbors = grid
            .empty_neighbors(hex)
            .iter()
            .filter_map(|hex| {
                let (hex_clamped, was_clamped) = clamp_inside_world_bounds(hex, &grid);
                match was_clamped {
                    true => None,
                    false => Some(hex_clamped),
                }
            })
            .collect::<Vec<Hex>>();
        // info!("empty_neighbors {:?}", empty_neighbors);
        empty_neighbors.sort_by(|a_hex, b_hex| {
            let a_hex = *a_hex;
            let b_hex = *b_hex;
            let a_hex_pos = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(a_hex));
            let b_hex_pos = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(b_hex));
            let a_distance = projectile_position.distance(a_hex_pos);
            let b_distance = projectile_position.distance(b_hex_pos);
            // println!(
            //     "a_hex({}, {}) a_pos({}, {}) a_dist({}) b_hex({}, {}) b_pos({}, {}) b_dist({})",
            //     a_hex.x,
            //     a_hex.y,
            //     a_hex_pos.x,
            //     a_hex_pos.y,
            //     a_distance,
            //     b_hex.x,
            //     b_hex.y,
            //     b_hex_pos.x,
            //     b_hex_pos.y,
            //     b_distance
            // );
            b_distance.total_cmp(&a_distance)
        });

        empty_neighbors = empty_neighbors
            .iter()
            .filter_map(|e_hex| {
                // println!("e_hex({}, {})", e_hex.x, e_hex.y);
                // get empty neighbors (free grid places)
                // filter by min and max column (do not overflow left and right column)
                // filter only that have neighbours in grid
                match grid.neighbors(*e_hex).len() > 0 {
                    true => Some(*e_hex),
                    false => None,
                }
            })
            .collect::<Vec<Hex>>();
        // info!(
        //     "empty_neighbors that have neighbors in grid {:?}",
        //     empty_neighbors
        // );
        let mut grid_hex_option = grid.get(hex);
        while grid_hex_option.is_some() && empty_neighbors.len() > 0 {
            // info!("found the same position in grid  hex({}, {})", hex.x, hex.y);
            if let Some(pop_hex) = empty_neighbors.pop() {
                hex = pop_hex;
                grid_hex_option = grid.get(hex);
            }
        }
        if grid_hex_option.is_some() {
            error!("Can not snap projectile to grid, all possible places occupied!");
            begin_turn.send(BeginTurn);
            return;
        }

        let no_neighbors = grid.neighbors(hex).len() == 0;
        let hex_pos = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(hex));
        info!(
            "final snap hex({}, {}) pos({}, {}) no_neighbors({})",
            hex.x, hex.y, hex_pos.x, hex_pos.y, no_neighbors
        );
        let ball = commands
            .spawn(GridBallBundle::new(
                hex_pos,
                grid.layout.hex_size.x,
                snap_projectile.species,
                &gameplay_meshes,
                &gameplay_materials,
                hex,
                match hex.y == 0 {
                    true => RigidBody::KinematicPositionBased,
                    false => RigidBody::Dynamic,
                },
            ))
            .with_children(|parent| {
                for joint in build_joints(hex, &grid) {
                    parent.spawn(joint);
                }
            })
            .id();

        grid.set(hex, ball); // add snapped projectile ball as grid ball
        let mut score_add = 0;

        if no_neighbors {
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
                                Vec2::new(
                                    ball_transform.translation.x,
                                    ball_transform.translation.y,
                                ),
                                grid.layout.hex_size.x,
                                *ball_species,
                                &gameplay_meshes,
                                &mut materials,
                            ));
                        } else if c_hex.x == hex.x && c_hex.y == hex.y {
                            commands.spawn(OutBallBundle::new(
                                hex_pos,
                                grid.layout.hex_size.x,
                                snap_projectile.species,
                                &gameplay_meshes,
                                &mut materials,
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
                                Vec2::new(
                                    ball_transform.translation.x,
                                    ball_transform.translation.y,
                                ),
                                grid.layout.hex_size.x,
                                *ball_species,
                                &gameplay_meshes,
                                &mut materials,
                            ));
                        } else if c_hex.x == hex.x && c_hex.y == hex.y {
                            commands.spawn(OutBallBundle::new(
                                hex_pos,
                                grid.layout.hex_size.x,
                                snap_projectile.species,
                                &gameplay_meshes,
                                &mut materials,
                            ));
                        }
                        commands.entity(*grid_ball_entity).despawn_recursive();
                    }
                    grid.remove(&c_hex);
                    score_add += 1;
                });

            if score_add > 0 {
                pkv_play_score_audio(&mut commands, &audio_assets, &pkv);
            }

            score_counter.0 += score_add;
            grid.print_sorted_axial();
            println!("len {} score_add {}", grid.storage.len(), score_add);
        }

        turn_counter.0 += 1;
        if score_add == 0 && cooldown_move_counter.init_value != 0 {
            cooldown_move_counter.value -= 1;
            if cooldown_move_counter.value == 0 {
                move_counter.0 += 1;
                cooldown_move_counter.value = cooldown_move_counter.init_value;
            }
        }

        begin_turn.send(BeginTurn);
    }
    snap_projectile_events.clear();
}

pub fn tick_collision_snap_cooldown_timer(
    mut commands: Commands,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    time: Res<Time>,
    mut projectile_query: Query<
        (Entity, &Transform, &mut ProjectileBall, &Species, &Velocity),
        With<ProjectileBall>,
    >,
    mut snap_projectile: EventWriter<SnapProjectile>,
) {
    if !collision_snap_cooldown.timer.paused() {
        collision_snap_cooldown.timer.tick(time.delta());
        if let Ok((
            projectile_entity,
            projectile_transform,
            mut projectile_ball,
            species,
            velocity,
        )) = projectile_query.get_single_mut()
        {
            if collision_snap_cooldown.is_ready_for_check(|| {
                return is_move_slow(velocity.linvel);
            }) {
                // snap projectile anyway after some time
                collision_snap_cooldown.restart();
                projectile_ball.is_ready_to_despawn = true;
                commands.entity(projectile_entity).despawn_recursive();
                info!("Projectile timeout snap");
                snap_projectile.send(SnapProjectile {
                    pos: Vec2::new(
                        projectile_transform.translation.x,
                        projectile_transform.translation.y,
                    ),
                    species: *species,
                });
            }
        }
    }
}
