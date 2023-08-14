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
use bevy_rapier2d::prelude::{CollisionEvent, ExternalImpulse, Velocity};
use hexx::{shapes, Hex};

use crate::{
    components::AppState,
    game_audio::utils::pkv_play_score_audio,
    gameplay::{
        ball::{
            components::{
                GridBall, GridBallAnimate, LastActiveGridBall, OutBall, ProjectileBall, Species,
            },
            events::SnapProjectile,
            grid_ball_bundle::GridBallBundle,
            out_ball_bundle::OutBallBundle,
        },
        constants::{MIN_CLUSTER_SIZE, MIN_PROJECTILE_SNAP_DOT, MOVE_DOWN_VELOCITY},
        events::{BeginTurn, UpdateCooldownCounter, UpdateMoveDown},
        grid::utils::{
            clamp_inside_world_bounds, find_cluster, find_floating_clusters, is_move_slow,
            remove_projectile, remove_projectile_and_snap,
        },
        lines::components::LineType,
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        panels::resources::{CooldownMoveCounter, MoveCounter, ScoreCounter, TurnCounter},
    },
    loading::audio_assets::AudioAssets,
    resources::LevelCounter,
};

use super::{
    resources::{CollisionSnapCooldown, Grid},
    utils::{adjust_grid_layout, build_revolute_joint, is_move_reverse},
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
    grid.calc_init_cols_rows(level_counter.0);
    adjust_grid_layout(&window_query, &mut grid, &MoveCounter(0));
    let max_side_x = grid.init_cols / 2;
    for hex in shapes::pointy_rectangle([-max_side_x, max_side_x, -grid.init_rows + 1, 0]) {
        let is_even = hex.y % 2 == 0;
        let offset = hex.to_offset_coordinates(grid.offset_mode);
        if (!is_even && offset[0] == max_side_x) || hex.y < grid.last_active_row {
            continue;
        }
        let is_last_active = hex.y == grid.last_active_row;

        let entity = GridBallBundle::spawn(
            &mut commands,
            grid.as_ref(),
            &gameplay_meshes,
            &gameplay_materials,
            hex,
            is_last_active,
            None,
        );
        grid.set(hex, entity);
    }
    app_state_next_state.set(AppState::Gameplay);
}

pub fn update_hex_coord_transforms(
    mut commands: Commands,
    mut balls_query: Query<
        (Entity, &mut GridBall),
        (
            With<LastActiveGridBall>,
            Without<GridBallAnimate>,
            Without<LineType>,
        ),
    >,
    mut grid: ResMut<Grid>,
    mut event_query: EventReader<UpdateMoveDown>,
    move_counter: Res<MoveCounter>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines_query: Query<(&LineType, &mut Transform), With<LineType>>,
) {
    if event_query.is_empty() {
        return;
    }
    event_query.clear();

    for (line_type, mut line_transform) in lines_query.iter_mut() {
        match line_type {
            LineType::GridTop => line_transform.translation.y = grid.bounds.maxs.y,
            LineType::GridBottom => line_transform.translation.y = grid.bounds.mins.y,
            LineType::GameOver => {}
        }
    }

    adjust_grid_layout(&window_query, &mut grid, &move_counter);
    for (ball_entity, grid_ball) in balls_query.iter_mut() {
        let hex = grid_ball.hex;
        let position = grid.layout.hex_to_world_pos(hex);
        commands
            .entity(ball_entity)
            .insert(GridBallAnimate { position });
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
    balls_query: Query<(&Transform, &Species, &mut GridBall), With<GridBall>>,
    audio_assets: Res<AudioAssets>,
    pkv: Res<PkvStore>,
    mut writer_update_cooldown_counter: EventWriter<UpdateCooldownCounter>,
) {
    if let Some(snap_projectile) = snap_projectile_events.iter().next() {
        grid.check_update_bounds();

        // println!("{}", grid.bounds);
        let projectile_position = snap_projectile.pos;
        // let projectile_position = Vec2::new(-1.0528764, 12.633377);
        let mut hex = grid.layout.world_pos_to_hex(projectile_position);
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
        let hex_pos = grid.layout.hex_to_world_pos(hex);
        info!(
            "final snap hex({}, {}) pos({}, {}) no_neighbors({})",
            hex.x, hex.y, hex_pos.x, hex_pos.y, no_neighbors
        );
        let entity = GridBallBundle::spawn(
            &mut commands,
            grid.as_ref(),
            &gameplay_meshes,
            &gameplay_materials,
            hex,
            hex.y == 0,
            Some(snap_projectile.species),
        );

        grid.set(hex, entity); // add snapped projectile ball as grid ball
        let mut score_add = 0;

        if no_neighbors {
            // projectile ball snapped with no neghbours
            // do not calc floating clusters
        } else {
            let (cluster, _) = find_cluster(&grid, hex, |&e| {
                e == entity
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
        if score_add == 0 {
            writer_update_cooldown_counter.send(UpdateCooldownCounter);
        }

        begin_turn.send(BeginTurn);
    }
    snap_projectile_events.clear();
}

pub fn update_cooldown_move_counter(
    mut update_cooldown_counter_events: EventReader<UpdateCooldownCounter>,
    mut cooldown_move_counter: ResMut<CooldownMoveCounter>,
    mut move_counter: ResMut<MoveCounter>,
    mut grid: ResMut<Grid>,
    mut writer_update_move_down: EventWriter<UpdateMoveDown>,
) {
    if let Some(_) = update_cooldown_counter_events.iter().next() {
        if cooldown_move_counter.init_value != 0 {
            cooldown_move_counter.value -= 1;
            if cooldown_move_counter.value == 0 {
                move_counter.0 += 1;
                grid.last_active_row -= 1;
                cooldown_move_counter.value = cooldown_move_counter.init_value;
                writer_update_move_down.send(UpdateMoveDown {});
            }
        }
    }
    update_cooldown_counter_events.clear();
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
    mut commands: Commands,
    mut grid_balls_query: Query<
        (Entity, &mut Transform, &mut GridBallAnimate, &mut Velocity),
        With<GridBallAnimate>,
    >,
) {
    for (ball_entity, mut grid_ball_transform, grid_ball_animate, mut velocity) in
        grid_balls_query.iter_mut()
    {
        let diff = grid_ball_transform.translation.y - grid_ball_animate.position.y;
        // println!("animate grid ball {} {:?}", diff, diff.length());
        if diff.abs() > 0.1 {
            velocity.linvel.y = -diff * MOVE_DOWN_VELOCITY;
        } else {
            grid_ball_transform.translation.y = grid_ball_animate.position.y;
            commands.entity(ball_entity).remove::<GridBallAnimate>();
        }
    }
}
