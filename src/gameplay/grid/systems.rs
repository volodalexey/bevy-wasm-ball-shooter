use bevy::{
    prelude::{
        error, info, Assets, BuildChildren, Commands, DespawnRecursiveExt, Entity, EventReader,
        EventWriter, Input, KeyCode, NextState, Query, Res, ResMut, Transform, Vec2, With, Without,
    },
    sprite::ColorMaterial,
    time::Time,
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::{CollisionEvent, ExternalImpulse, RigidBody, Velocity};
use hexx::{shapes, Hex};

use crate::{
    components::AppState,
    game_audio::utils::pkv_play_score_audio,
    gameplay::{
        ball::{
            components::{GridBall, OutBall, ProjectileBall, Species},
            events::SnapProjectile,
            grid_ball_bundle::GridBallBundle,
            out_ball_bundle::OutBallBundle,
        },
        constants::{MIN_CLUSTER_SIZE, MIN_PROJECTILE_SNAP_DOT},
        events::BeginTurn,
        grid::utils::{
            build_ball_text, clamp_inside_world_bounds, find_cluster, find_floating_clusters,
            is_move_slow, remove_projectile, remove_projectile_and_snap,
        },
        lines::components::LineType,
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
    utils::{adjust_grid_layout, build_joints, build_revolute_joint, is_move_reverse},
};

pub fn generate_grid(
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut grid: ResMut<Grid>,
    level_counter: Res<LevelCounter>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    (grid.init_cols, grid.init_rows) = calc_init_cols_rows(&level_counter);
    adjust_grid_layout(&window_query, &mut grid, &MoveCounter(0));
    for hex in shapes::pointy_rectangle([0, grid.init_cols - 1, 0, grid.init_rows - 1]) {
        let hex_pos = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(hex));
        let is_first = hex.y == 0;

        let grid_ball_bundle = GridBallBundle::new(
            hex_pos,
            hex_pos,
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
            let entity = commands
                .spawn(grid_ball_bundle)
                .with_children(|parent| {
                    build_ball_text(parent, hex);
                })
                .id();

            grid.set(hex, entity);
            continue;
        }

        let entity = commands
            .spawn(grid_ball_bundle)
            .with_children(|parent| {
                for joint in build_joints(hex, &grid) {
                    parent.spawn(joint);
                }
                build_ball_text(parent, hex);
            })
            .id();
        grid.set(hex, entity);
    }
    app_state_next_state.set(AppState::Gameplay);
}

pub fn update_hex_coord_transforms(
    mut balls_query: Query<&mut GridBall, With<GridBall>>,
    mut grid: ResMut<Grid>,
    mut event_query: EventReader<UpdatePositions>,
    move_counter: Res<MoveCounter>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines_query: Query<(&LineType, &mut Transform), With<LineType>>,
) {
    if event_query.is_empty() {
        return;
    }
    event_query.clear();

    adjust_grid_layout(&window_query, &mut grid, &move_counter);
    grid.check_update_bounds();

    for (line_type, mut line_transform) in lines_query.iter_mut() {
        match line_type {
            LineType::GridTop => line_transform.translation.y = grid.bounds.maxs.y,
            LineType::GridBottom => line_transform.translation.y = grid.bounds.mins.y,
            LineType::GameOver => {}
        }
    }

    for mut grid_ball in balls_query.iter_mut() {
        let hex = grid_ball.hex;
        let pos_2d = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(hex));
        grid_ball.animation_x = pos_2d.x;
        grid_ball.animation_y = pos_2d.y;
        // println!(
        //     "pos_2d({}, {}) animation_y {}",
        //     pos_2d.x, pos_2d.y, grid_ball.animation_y
        // );
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
        if !projectile_ball.is_flying || projectile_ball.is_ready_to_despawn {
            return;
        }
        grid.check_update_bounds();
        if projectile_transform.translation.y > grid.bounds.maxs.y - grid.layout.hex_size.y {
            info!(
                "Projectile out of grid snap {} {}",
                grid.bounds.mins.y, projectile_transform.translation.y
            );
            remove_projectile(&mut commands, &projectile_entity, &mut projectile_ball);
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
        (
            Entity,
            &mut Transform,
            &Species,
            &mut Velocity,
            &mut ProjectileBall,
        ),
        With<ProjectileBall>,
    >,
    balls_query: Query<(Entity, &Transform, &GridBall), (With<GridBall>, Without<ProjectileBall>)>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    grid: Res<Grid>,
) {
    for (entity_a, entity_b, started) in collision_events.iter().map(|e| match e {
        CollisionEvent::Started(a, b, _) => (a, b, true),
        CollisionEvent::Stopped(a, b, _) => (a, b, false),
    }) {
        if let Ok((ball_entity, ball_transform, grid_ball)) =
            balls_query.get(*entity_a).or(balls_query.get(*entity_b))
        {
            let mut p1 = projectile_query.get_mut(*entity_a);
            if p1.is_err() {
                p1 = projectile_query.get_mut(*entity_b);
            }

            if let Ok((
                projectile_entity,
                projectile_transform,
                species,
                projectile_velocity,
                mut projectile_ball,
            )) = p1
            {
                // println!(
                //     "\ncollision phase {} with hex({}, {})\n",
                //     started, grid_ball.hex.x, grid_ball.hex.y
                // );
                // take into account only collision between projectile and grid ball
                // println!("velocity {:?} len {}", velocity, velocity.linvel.length());
                if !projectile_ball.is_ready_to_despawn
                    && match started {
                        true => {
                            // println!("Snap event {:?}", projectile_ball.snap_to);
                            if projectile_ball.snap_to.len() == 0 {
                                // snap with revolute joint only to the first grid ball
                                // if !projectile_ball.snap_to.contains(&grid_ball.hex) {
                                let anchor_pos = ball_transform.translation.truncate();
                                let from_pos = projectile_transform.translation.truncate();
                                let diff = (anchor_pos - from_pos).normalize();
                                let vel = projectile_velocity.linvel.normalize();
                                let dot = vel.dot(diff);
                                // println!(
                                //     "diff({}, {}) vel({}, {}) dot({})",
                                //     diff.x, diff.y, vel.x, vel.y, dot
                                // );
                                if dot > MIN_PROJECTILE_SNAP_DOT {
                                    collision_snap_cooldown.start();
                                    // save first touch position
                                    is_move_reverse(
                                        &mut projectile_ball,
                                        projectile_velocity.linvel,
                                    );
                                    commands.entity(projectile_entity).with_children(|parent| {
                                        parent.spawn(build_revolute_joint(
                                            &ball_entity,
                                            anchor_pos,
                                            from_pos,
                                            true,
                                        ));
                                    });
                                    projectile_ball.snap_to.push(grid_ball.hex);
                                }
                                false
                            } else {
                                is_move_reverse(&mut projectile_ball, projectile_velocity.linvel)
                            }
                        }
                        false => {
                            is_move_slow(projectile_velocity.linvel)
                                || is_move_reverse(&mut projectile_ball, projectile_velocity.linvel)
                        }
                    }
                {
                    collision_snap_cooldown.stop();
                    // if ball turned back
                    // or ball moves too slow
                    info!("Projectile too slow so snap");
                    snap_projectile.send(SnapProjectile {
                        pos: remove_projectile_and_snap(
                            &mut commands,
                            &projectile_entity,
                            &projectile_transform,
                            &mut projectile_ball,
                            &grid,
                            &balls_query,
                        ),
                        species: *species,
                    });
                }
            }
        }
    }
}

pub fn control_projectile_position(
    keyboard_input_key_code: Res<Input<KeyCode>>,
    mut projectile_query: Query<&mut ExternalImpulse, With<ProjectileBall>>,
) {
    for mut projectile_impulse in projectile_query.iter_mut() {
        let mut direction = Vec2::ZERO;
        if keyboard_input_key_code.any_pressed([KeyCode::Left]) {
            direction += Vec2::new(-1.0, 0.0);
        }
        if keyboard_input_key_code.any_pressed([KeyCode::Right]) {
            direction += Vec2::new(1.0, 0.0);
        }
        if keyboard_input_key_code.any_pressed([KeyCode::Up]) {
            direction += Vec2::new(0.0, 1.0);
        }
        if keyboard_input_key_code.any_pressed([KeyCode::Down]) {
            direction += Vec2::new(0.0, -1.0);
        }

        direction = direction.normalize_or_zero();
        if direction.length() > 0.0 {
            projectile_impulse.impulse = direction * 10.0;
            // println!("apply impulse {:?}", projectile_impulse.impulse);
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
        info!(
            "snap hex({}, {}) pos({}, {})",
            hex.x, hex.y, projectile_position.x, projectile_position.y
        );
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
        info!("empty_neighbors {:?}", empty_neighbors);

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
        info!(
            "empty_neighbors that have neighbors in grid {:?}",
            empty_neighbors
        );
        grid.sort_neighbors(&mut empty_neighbors, projectile_position);
        info!("empty_neighbors sorted by distance {:?}", empty_neighbors);

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
                projectile_position,
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
                build_ball_text(parent, hex);
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
                                *ball_species,
                                &gameplay_meshes,
                                &mut materials,
                            ));
                        } else if c_hex.x == hex.x && c_hex.y == hex.y {
                            commands.spawn(OutBallBundle::new(
                                hex_pos,
                                snap_projectile.species,
                                &gameplay_meshes,
                                &mut materials,
                            ));
                        }
                        commands.entity(*grid_ball_entity).despawn_recursive();
                    }
                    println!("removed hex({}, {})", c_hex.x, c_hex.y);
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
                                *ball_species,
                                &gameplay_meshes,
                                &mut materials,
                            ));
                        } else if c_hex.x == hex.x && c_hex.y == hex.y {
                            commands.spawn(OutBallBundle::new(
                                hex_pos,
                                snap_projectile.species,
                                &gameplay_meshes,
                                &mut materials,
                            ));
                        }
                        commands.entity(*grid_ball_entity).despawn_recursive();
                    }
                    println!("removed hex({}, {})", c_hex.x, c_hex.y);
                    grid.remove(&c_hex);
                    score_add += 1;
                });

            if score_add > 0 {
                pkv_play_score_audio(&mut commands, &audio_assets, &pkv);
            }

            score_counter.0 += score_add;
            // grid.print_sorted_axial();
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
    balls_query: Query<(Entity, &Transform, &GridBall), (With<GridBall>, Without<ProjectileBall>)>,
    grid: Res<Grid>,
) {
    if !collision_snap_cooldown.timer.paused() {
        collision_snap_cooldown.timer.tick(time.delta());
        if let Ok((
            projectile_entity,
            projectile_transform,
            mut projectile_ball,
            species,
            projectile_velocity,
        )) = projectile_query.get_single_mut()
        {
            if collision_snap_cooldown.is_ready_for_check(|| {
                is_move_slow(projectile_velocity.linvel)
                    || is_move_reverse(&mut projectile_ball, projectile_velocity.linvel)
            }) {
                // snap projectile anyway after some time
                collision_snap_cooldown.stop();
                info!("Projectile cooldown snap");
                snap_projectile.send(SnapProjectile {
                    pos: remove_projectile_and_snap(
                        &mut commands,
                        &projectile_entity,
                        &projectile_transform,
                        &mut projectile_ball,
                        &grid,
                        &balls_query,
                    ),
                    species: *species,
                });
            }
        }
    }
}

pub fn animate_grid_ball(
    mut grid_balls_query: Query<(&mut Transform, &mut GridBall), With<GridBall>>,
) {
    for (mut transform, grid_ball) in grid_balls_query.iter_mut() {
        let diff_x = grid_ball.animation_x - transform.translation.x;
        if diff_x.abs() > 0.01 {
            // println!(
            //     "grid_ball.animation_y {} transform.translation.y {} diff {}",
            //     grid_ball.animation_y,
            //     transform.translation.y,
            //     diff_y.abs()
            // );
            if diff_x.abs() < 0.1 {
                // println!("set final {}", grid_ball.animation_y);
                transform.translation.x = grid_ball.animation_x;
            } else {
                transform.translation.x = transform.translation.x + diff_x * 0.1;
            }
        }
        let diff_y = grid_ball.animation_y - transform.translation.y;
        if diff_y.abs() > 0.01 {
            // println!(
            //     "grid_ball.animation_y {} transform.translation.y {} diff {}",
            //     grid_ball.animation_y,
            //     transform.translation.y,
            //     diff_y.abs()
            // );
            if diff_y.abs() < 0.1 {
                // println!("set final {}", grid_ball.animation_y);
                transform.translation.y = grid_ball.animation_y;
            } else {
                transform.translation.y = transform.translation.y + diff_y * 0.1;
            }
        }
    }
}
