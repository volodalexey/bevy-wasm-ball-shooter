use bevy::{
    prelude::{
        info, Assets, BuildChildren, Commands, DespawnRecursiveExt, Entity, EventReader,
        EventWriter, Input, KeyCode, NextState, Query, Res, ResMut, Transform, Vec2, With, Without,
    },
    sprite::ColorMaterial,
    time::Time,
    utils::HashMap,
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
                GridBall, GridBallPositionAnimate, LastActiveGridBall, OutBall, ProjectileBall,
                Species,
            },
            events::SnapProjectile,
            grid_ball_bundle::GridBallBundle,
            out_ball_bundle::OutBallBundle,
        },
        constants::{
            BUILD_JOINT_TOLERANCE, FILL_PLAYGROUND_ROWS, MIN_CLUSTER_SIZE, MIN_PROJECTILE_SNAP_DOT,
            MOVE_DOWN_TOLERANCE, ROW_HEIGHT,
        },
        events::{
            BeginTurn, CheckJoints, FindCluster, SpawnRow, UpdateMoveDown, UpdateScoreCounter,
        },
        grid::utils::{
            buid_cell_storage, build_connection_storage, find_cluster, find_floating_clusters,
            is_move_slow, remove_projectile,
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
    utils::{
        adjust_grid_layout, build_ball_text, build_corners_joints, build_prismatic_joint,
        build_revolute_joint, is_move_reverse,
    },
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
    let mut spawned: HashMap<Hex, Entity> = HashMap::default();
    for hex in shapes::pointy_rectangle([-max_side_x, max_side_x, -grid.init_rows + 1, 0]) {
        let is_even = hex.y % 2 == 0;
        let offset = hex.to_offset_coordinates(grid.offset_mode);
        if (!is_even && offset[0] == max_side_x) || hex.y < grid.last_active_row {
            continue;
        }
        let is_last_active = hex.y == grid.last_active_row;

        let (new_entity, _) = GridBallBundle::spawn(
            &mut commands,
            &gameplay_meshes,
            &gameplay_materials,
            grid.layout.hex_to_world_pos(hex),
            is_last_active,
            None,
            true,
        );

        spawned.insert(hex, new_entity);
    }
    // build connections and joints
    for (hex, entity) in spawned.iter() {
        let hex_pos = grid.layout.hex_to_world_pos(*hex);

        let mut entity_commands = commands.entity(*entity);

        entity_commands.with_children(|parent| {
            build_ball_text(parent, Some(*hex));
            if hex.y != grid.last_active_row {
                // find available hex neighbors for current hex
                for (neighbor_hex, neighbor_entity) in
                    hex.all_neighbors().iter().filter_map(|&neighbor_hex| {
                        match spawned.get(&neighbor_hex) {
                            Some(neighbor_entity) => Some((neighbor_hex, neighbor_entity)),
                            None => None,
                        }
                    })
                {
                    let neighbor_pos = grid.layout.hex_to_world_pos(neighbor_hex);

                    parent.spawn(build_prismatic_joint(
                        hex_pos,
                        neighbor_pos,
                        *neighbor_entity,
                    ));
                }
            }
        });
    }
    app_state_next_state.set(AppState::Gameplay);
}

pub fn update_hex_coord_transforms(
    mut commands: Commands,
    mut balls_query: Query<
        (Entity, &Transform),
        (
            With<LastActiveGridBall>,
            Without<GridBallPositionAnimate>,
            Without<LineType>,
        ),
    >,
    mut grid: ResMut<Grid>,
    mut move_down_events: EventReader<UpdateMoveDown>,
    move_counter: Res<MoveCounter>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines_query: Query<(&LineType, &mut Transform), With<LineType>>,
) {
    if move_down_events.is_empty() {
        return;
    }
    move_down_events.clear();
    adjust_grid_layout(&window_query, &mut grid, &move_counter);
    let hex = Hex {
        x: 0,
        y: grid.last_active_row,
    };
    let position = grid.layout.hex_to_world_pos(hex);

    for (line_type, mut line_transform) in lines_query.iter_mut() {
        match line_type {
            LineType::GridTop => line_transform.translation.y = position.y,
            LineType::GameOver => {}
        }
    }

    for (ball_entity, ball_transform) in balls_query.iter_mut() {
        let position = ball_transform.translation.truncate() - Vec2::new(0.0, ROW_HEIGHT);
        commands
            .entity(ball_entity)
            .insert(GridBallPositionAnimate::from_position(position));
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
    grid: Res<Grid>,
    mut snap_projectile: EventWriter<SnapProjectile>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
) {
    if let Ok((projectile_entity, projectile_transform, mut projectile_ball, species)) =
        projectile_query.get_single_mut()
    {
        if !projectile_ball.is_flying || projectile_ball.is_ready_to_despawn {
            return;
        }
        let hex = Hex {
            x: 0,
            y: grid.last_active_row,
        };
        let position = grid.layout.hex_to_world_pos(hex);
        let projectile_position = projectile_transform.translation.truncate();
        if projectile_position.y > position.y {
            info!(
                "Projectile out of grid snap {} {}",
                position.y, projectile_position.y
            );
            let corrected_hex = grid.layout.world_pos_to_hex(projectile_position);
            let mut offset = corrected_hex.to_offset_coordinates(grid.offset_mode);
            if offset[1] != grid.last_active_row {
                offset[1] = grid.last_active_row;
            }
            let corrected_position = grid
                .layout
                .hex_to_world_pos(Hex::from_offset_coordinates(offset, grid.offset_mode));
            remove_projectile(&mut commands, &projectile_entity, &mut projectile_ball);
            collision_snap_cooldown.stop();
            snap_projectile.send(SnapProjectile {
                pos: projectile_position,
                cor_pos: corrected_position,
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
    balls_query: Query<(Entity, &Transform), (With<GridBall>, Without<ProjectileBall>)>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    mut writer_check_joints: EventWriter<CheckJoints>,
) {
    for (entity_a, entity_b, started) in collision_events.iter().map(|e| match e {
        CollisionEvent::Started(a, b, _) => (a, b, true),
        CollisionEvent::Stopped(a, b, _) => (a, b, false),
    }) {
        let some_ball_entity_a = balls_query.get(*entity_a);
        let some_ball_entity_b = balls_query.get(*entity_b);
        if let Ok((ball_entity, ball_transform)) = some_ball_entity_a.or(some_ball_entity_b) {
            if some_ball_entity_a.is_ok() && some_ball_entity_b.is_ok() {
                writer_check_joints.send(CheckJoints {
                    start_from: ball_entity,
                });
            }
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
                // take into account only collision between projectile and grid ball
                if !projectile_ball.is_ready_to_despawn
                    && match started {
                        true => {
                            if projectile_ball.snap_to.len() == 0 {
                                // snap with revolute joint only to the first grid ball
                                let anchor_pos = ball_transform.translation.truncate();
                                let from_pos = projectile_transform.translation.truncate();
                                let diff = (anchor_pos - from_pos).normalize();
                                let vel = projectile_velocity.linvel.normalize();
                                let dot = vel.dot(diff);
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
                                    projectile_ball.snap_to.push(ball_entity);
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
                    remove_projectile(&mut commands, &projectile_entity, &mut projectile_ball);
                    let projectile_position = projectile_transform.translation.truncate();
                    snap_projectile.send(SnapProjectile {
                        pos: projectile_position,
                        cor_pos: projectile_position,
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
    grid: Res<Grid>,
    mut begin_turn: EventWriter<BeginTurn>,
    mut turn_counter: ResMut<TurnCounter>,
    balls_query: Query<(Entity, &Transform, &Species, &GridBall), With<GridBall>>,
    mut writer_find_cluster: EventWriter<FindCluster>,
) {
    if let Some(snap_projectile) = snap_projectile_events.iter().next() {
        let projectile_position = snap_projectile.pos;
        let possible_hex = grid.layout.world_pos_to_hex(projectile_position);
        let is_last_active = possible_hex.y == grid.last_active_row;
        let (new_entity, _) = GridBallBundle::spawn(
            &mut commands,
            &gameplay_meshes,
            &gameplay_materials,
            projectile_position,
            is_last_active,
            Some(snap_projectile.species),
            false,
        );

        if !is_last_active {
            build_corners_joints(
                &mut commands,
                &grid,
                new_entity,
                snap_projectile.cor_pos,
                &balls_query
                    .iter()
                    .map(|(neighbor_entity, neighbor_transform, _, _)| {
                        (neighbor_entity, neighbor_transform.translation.truncate())
                    })
                    .collect::<Vec<(Entity, Vec2)>>(),
            );
        }
        if projectile_position != snap_projectile.cor_pos {
            commands
                .entity(new_entity)
                .insert(GridBallPositionAnimate::from_position(
                    snap_projectile.cor_pos,
                ));
        }

        turn_counter.0 += 1;

        begin_turn.send(BeginTurn);
        writer_find_cluster.send(FindCluster {
            start_from: new_entity,
        });
    }
    snap_projectile_events.clear();
}

pub fn find_and_remove_clusters(
    mut commands: Commands,
    mut find_cluster_events: EventReader<FindCluster>,
    balls_query: Query<(Entity, &Transform, &Species, Option<&LastActiveGridBall>), With<GridBall>>,
    gameplay_meshes: Res<GameplayMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut writer_update_cooldown_counter: EventWriter<UpdateScoreCounter>,
) {
    if find_cluster_events.is_empty() {
        return;
    }
    // println!("balls_query total {}", balls_query.iter().len());
    for FindCluster { start_from } in find_cluster_events.iter() {
        let cell_storage = buid_cell_storage(&balls_query);

        let mut connection_storage = build_connection_storage(&balls_query, &cell_storage);

        let (cluster, _) = find_cluster(*start_from, &connection_storage, true);

        let mut cluster_score_add = 0;
        if cluster.len() >= MIN_CLUSTER_SIZE {
            println!(
                "cluster {:?} connection_storage before {}",
                cluster,
                connection_storage.len()
            );
            // remove matching cluster
            cluster.iter().for_each(|(cluster_entity, _)| {
                if let Ok((cluster_entity, cluster_transform, cluster_species, _)) =
                    balls_query.get(*cluster_entity)
                {
                    commands.spawn(OutBallBundle::new(
                        cluster_transform.translation.truncate(),
                        *cluster_species,
                        &gameplay_meshes,
                        &mut materials,
                        false,
                    ));
                    commands.entity(cluster_entity).despawn_recursive();
                    println!(
                        "removed cluster entity {} {}",
                        cluster_entity.index(),
                        *cluster_species
                    );
                    connection_storage.remove(&cluster_entity);
                    cluster_score_add += 1;
                }
            });
        }
        println!(
            "score_add for cluster {:?} connection_storage after {}",
            cluster_score_add,
            connection_storage.len()
        );

        let mut floating_clusters_score_add = 0;
        let floating_clusters = find_floating_clusters(&connection_storage);
        // remove floating clusters
        floating_clusters
            .iter()
            .flat_map(|e| e.iter())
            .for_each(|(entity, _)| {
                if let Ok((cluster_entity, cluster_transform, cluster_species, _)) =
                    balls_query.get(*entity)
                {
                    commands.spawn(OutBallBundle::new(
                        cluster_transform.translation.truncate(),
                        *cluster_species,
                        &gameplay_meshes,
                        &mut materials,
                        true,
                    ));
                    commands.entity(cluster_entity).despawn_recursive();
                    println!(
                        "removed floating cluster entity {} {}",
                        cluster_entity.index(),
                        *cluster_species
                    );
                    floating_clusters_score_add += 2;
                }
            });

        println!(
            "score_add for floating clusters {:?}",
            floating_clusters_score_add
        );

        let score_add = cluster_score_add + floating_clusters_score_add;

        println!("score_add {}", score_add);

        writer_update_cooldown_counter.send(UpdateScoreCounter { score_add });
    }
}

pub fn update_score_counter(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    pkv: Res<PkvStore>,
    mut update_cooldown_counter_events: EventReader<UpdateScoreCounter>,
    mut cooldown_move_counter: ResMut<CooldownMoveCounter>,
    mut move_counter: ResMut<MoveCounter>,
    mut score_counter: ResMut<ScoreCounter>,
    mut writer_update_move_down: EventWriter<UpdateMoveDown>,
) {
    if let Some(UpdateScoreCounter { score_add }) = update_cooldown_counter_events.iter().next() {
        if *score_add > 0 {
            pkv_play_score_audio(&mut commands, &audio_assets, &pkv);
            score_counter.0 += score_add;
        } else if cooldown_move_counter.init_value != 0 {
            cooldown_move_counter.value -= 1;
            if cooldown_move_counter.value == 0 {
                move_counter.0 += 1;
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
                remove_projectile(&mut commands, &projectile_entity, &mut projectile_ball);
                let projectile_position = projectile_transform.translation.truncate();
                snap_projectile.send(SnapProjectile {
                    pos: projectile_position,
                    cor_pos: projectile_position,
                    species: *species,
                });
            }
        }
    }
}

pub fn animate_grid_ball_position(
    mut commands: Commands,
    mut grid_balls_query: Query<
        (Entity, &mut Transform, &mut GridBallPositionAnimate),
        With<GridBallPositionAnimate>,
    >,
    time: Res<Time>,
    grid: Res<Grid>,
    move_counter: Res<MoveCounter>,
    mut writer_spawn_row: EventWriter<SpawnRow>,
) {
    let total_count = grid_balls_query.iter().len();
    let mut completed_count: usize = 0;
    for (ball_entity, mut grid_ball_transform, mut grid_ball_animate) in grid_balls_query.iter_mut()
    {
        grid_ball_animate.timer.tick(time.delta());
        grid_ball_transform.translation = grid_ball_transform
            .translation
            .truncate()
            .lerp(
                grid_ball_animate.position,
                grid_ball_animate.timer.percent(),
            )
            .extend(grid_ball_transform.translation.z);
        if (grid_ball_transform.translation.truncate() - grid_ball_animate.position).length()
            < MOVE_DOWN_TOLERANCE
        {
            grid_ball_transform.translation = grid_ball_animate
                .position
                .extend(grid_ball_transform.translation.z);
            commands
                .entity(ball_entity)
                .remove::<GridBallPositionAnimate>();
            completed_count += 1;
        }
    }
    if completed_count == total_count && total_count > 0 {
        if grid.init_rows - FILL_PLAYGROUND_ROWS > move_counter.0 as i32 - 1 {
            writer_spawn_row.send(SpawnRow);
        }
    }
}

pub fn on_spawn_row(
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut spawn_row_events: EventReader<SpawnRow>,
    mut grid: ResMut<Grid>,
    mut grid_balls_query: Query<(Entity, &Transform, &Species, Option<&LastActiveGridBall>)>,
) {
    if spawn_row_events.is_empty() {
        return;
    }
    spawn_row_events.clear();

    grid.last_active_row -= 1;
    let max_side_x = grid.init_cols / 2;
    let mut spawned: Vec<(Entity, Vec2)> = vec![];
    let mut spawned_y: f32 = 0.0;
    for hex_x in -max_side_x..=max_side_x {
        let is_even = grid.last_active_row % 2 == 0;
        let hex = Hex::from_offset_coordinates([hex_x, grid.last_active_row], grid.offset_mode);
        let offset = hex.to_offset_coordinates(grid.offset_mode);
        if (!is_even && offset[0] == max_side_x) || hex.y < grid.last_active_row {
            continue;
        }
        let position = grid.layout.hex_to_world_pos(hex);
        spawned_y = position.y;
        let (new_entity, _) = GridBallBundle::spawn(
            &mut commands,
            &gameplay_meshes,
            &gameplay_materials,
            position,
            true,
            None,
            true,
        );
        commands
            .entity(new_entity)
            .with_children(|parent| build_ball_text(parent, Some(hex)));
        spawned.push((new_entity, position));
    }

    // we need to re-created previous last row, rapied physics issues because of changing RigidBody Kinematic => Dynamic
    let mut old_entities = grid_balls_query
        .iter()
        .filter_map(|(ball_entity, ball_transform, _, some_ball_last_active)| {
            let ball_position = ball_transform.translation.truncate();
            if some_ball_last_active.is_none()
                && (spawned_y - ball_position.y).abs() < BUILD_JOINT_TOLERANCE * 2.0
            {
                return Some((ball_entity, ball_position));
            }
            None
        })
        .collect::<Vec<(Entity, Vec2)>>();
    old_entities.extend(spawned);

    for (ball_entity, ball_transform, ball_species, some_ball_last_active) in
        grid_balls_query.iter_mut()
    {
        if some_ball_last_active.is_some() {
            commands.entity(ball_entity).despawn_recursive();
            let ball_position = ball_transform.translation.truncate();
            let (new_entity, _) = GridBallBundle::spawn(
                &mut commands,
                &gameplay_meshes,
                &gameplay_materials,
                ball_position,
                false,
                Some(*ball_species),
                false,
            );
            commands
                .entity(new_entity)
                .with_children(|parent| build_ball_text(parent, None));

            build_corners_joints(
                &mut commands,
                &grid,
                new_entity,
                ball_position,
                &old_entities,
            );
        }
    }
}
