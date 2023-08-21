use bevy::{
    prelude::{
        info, Assets, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, Input,
        KeyCode, NextState, Query, Res, ResMut, Transform, Vec2, With, Without,
    },
    sprite::ColorMaterial,
    time::Time,
    utils::{HashMap, HashSet},
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::{CollisionEvent, ExternalForce, Velocity};
use hexx::{shapes, Hex};

use crate::{
    components::AppState,
    game_audio::utils::pkv_play_score_audio,
    gameplay::{
        ball::{
            components::{
                GridBall, GridBallPositionAnimate, GridBallScaleAnimate, LastActiveGridBall,
                MagneticGridBall, OutBall, ProjectileBall, Species,
            },
            grid_ball_bundle::GridBallBundle,
            out_ball_bundle::OutBallBundle,
        },
        constants::{
            BUILD_JOINT_TOLERANCE, FILL_PLAYGROUND_ROWS, MAGNETIC_DISTANCE_STRONG,
            MAGNETIC_DISTANCE_WEAK, MAGNETIC_FACTOR_STRONG, MAGNETIC_FACTOR_WEAK,
            MAX_GRID_BALL_SPEED, MIN_CLUSTER_SIZE, MIN_PROJECTILE_SNAP_DOT, MOVE_DOWN_TOLERANCE,
            ROW_HEIGHT,
        },
        events::{
            FindCluster, MoveDownLastActive, ProjectileReload, SnapProjectile, SpawnRow,
            UpdateScoreCounter,
        },
        grid::utils::{
            buid_cells_to_entities, build_entities_to_neighbours, find_cluster, is_move_slow,
            send_snap_projectile,
        },
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        panels::resources::{CooldownMoveCounter, MoveCounter, ScoreCounter, TurnCounter},
    },
    loading::audio_assets::AudioAssets,
    resources::LevelCounter,
};

use super::{
    resources::{ClusterCheckCooldown, CollisionSnapCooldown, Grid},
    utils::{adjust_grid_layout, is_move_reverse},
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
    let mut spawned: Vec<(Entity, Vec2)> = vec![];
    let mut to_process: Vec<(Entity, Vec2)> = vec![];
    for hex in shapes::pointy_rectangle([-max_side_x, max_side_x, -grid.init_rows + 1, 0]) {
        let is_even = hex.y % 2 == 0;
        let offset = hex.to_offset_coordinates(grid.offset_mode);
        if (!is_even && offset[0] == max_side_x) || hex.y < grid.last_active_row {
            continue;
        }
        let is_last_active = hex.y == grid.last_active_row;

        let position = grid.layout.hex_to_world_pos(hex);
        let (new_entity, _) = GridBallBundle::spawn(
            &mut commands,
            &gameplay_meshes,
            &gameplay_materials,
            position,
            is_last_active,
            true,
            false,
            None,
            true,
            true,
        );

        spawned.push((new_entity, position));
        if !is_last_active {
            to_process.push((new_entity, position));
        }
    }
    app_state_next_state.set(AppState::Gameplay);
}

pub fn move_down_last_active(
    mut commands: Commands,
    mut balls_query: Query<
        (Entity, &Transform, Option<&GridBallPositionAnimate>),
        With<LastActiveGridBall>,
    >,
    mut grid: ResMut<Grid>,
    mut move_down_events: EventReader<MoveDownLastActive>,
    move_counter: Res<MoveCounter>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if move_down_events.is_empty() {
        return;
    }
    move_down_events.clear();
    adjust_grid_layout(&window_query, &mut grid, &move_counter);

    for (ball_entity, ball_transform, some_ball_animate) in balls_query.iter_mut() {
        let position = match some_ball_animate {
            Some(ball_animate) => ball_animate.position,
            None => ball_transform.translation.truncate(),
        } - Vec2::new(0.0, ROW_HEIGHT);
        commands
            .entity(ball_entity)
            .insert(GridBallPositionAnimate::from_position(position, true));
    }
}

pub fn apply_magnetic_forces(
    mut magnetic_balls_query: Query<
        (
            Entity,
            &mut Transform,
            &GridBall,
            &mut ExternalForce,
            &mut Velocity,
            Option<&GridBallPositionAnimate>,
            Option<&GridBallScaleAnimate>,
        ),
        (With<MagneticGridBall>, Without<ProjectileBall>),
    >,
    grid: Res<Grid>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    let hex = Hex {
        x: 0,
        y: grid.last_active_row,
    };
    let last_active_position = grid.layout.hex_to_world_pos(hex);
    let mut entities_to_positions: HashMap<Entity, Vec2> = HashMap::default();
    magnetic_balls_query
        .iter()
        .for_each(|(e, t, gb, _, _, _, _)| {
            if !gb.is_ready_to_despawn {
                entities_to_positions.insert(e, t.translation.truncate());
            }
        });
    for (
        entity,
        mut transform,
        _,
        mut external_force,
        velocity,
        grid_ball_animate_position,
        grid_ball_animate_scale,
    ) in magnetic_balls_query.iter_mut()
    {
        if grid_ball_animate_position.is_some() || grid_ball_animate_scale.is_some() {
            // only for magnetic
            continue;
        }
        let mut result_acc_strong = Vec2::ZERO;
        let mut result_acc_weak = Vec2::ZERO;
        let position = transform.translation.truncate();
        for (neighbour, neighbour_position) in entities_to_positions.iter() {
            if *neighbour == entity {
                continue;
            }
            let direction = *neighbour_position - position;
            let dist = position.distance(*neighbour_position);
            if dist < MAGNETIC_DISTANCE_STRONG {
                result_acc_strong += direction;
            } else if dist < MAGNETIC_DISTANCE_WEAK {
                result_acc_weak += direction;
            }
        }
        external_force.force = result_acc_strong.normalize_or_zero() * MAGNETIC_FACTOR_STRONG
            + result_acc_weak.normalize_or_zero() * MAGNETIC_FACTOR_WEAK;
        velocity.linvel.clamp_length_max(MAX_GRID_BALL_SPEED);
        if keyboard_input_key_code.any_pressed([KeyCode::L]) {
            println!(
                "[len {}] force {} velocity {} position {} last_active_position {}",
                entities_to_positions.len(),
                external_force.force,
                velocity.linvel,
                position.y,
                last_active_position.y
            );
        }
        // confine grid ball position
        if position.y > last_active_position.y {
            transform.translation.y = last_active_position.y;
        }
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
    projectile_query: Query<(Entity, &Transform, &ProjectileBall), With<ProjectileBall>>,
    grid: Res<Grid>,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
) {
    for (projectile_entity, projectile_transform, projectile_ball) in projectile_query.iter() {
        if !projectile_ball.is_flying {
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
            send_snap_projectile(
                collision_snap_cooldown.as_mut(),
                &mut writer_snap_projectile,
                projectile_entity,
            );
        }
    }
}

pub fn check_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
    mut projectile_query: Query<
        (Entity, &mut Transform, &mut Velocity, &mut ProjectileBall),
        With<ProjectileBall>,
    >,
    balls_query: Query<
        (Entity, &Transform),
        (
            With<GridBall>,
            Without<ProjectileBall>,
            Without<GridBallScaleAnimate>,
        ),
    >,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    mut writer_find_cluster: EventWriter<FindCluster>,
    time: Res<Time>,
    mut cluster_check_cooldown: ResMut<ClusterCheckCooldown>,
) {
    cluster_check_cooldown.timer.tick(time.delta());
    for (entity_a, entity_b, started) in collision_events.iter().map(|e| match e {
        CollisionEvent::Started(a, b, _) => (a, b, true),
        CollisionEvent::Stopped(a, b, _) => (a, b, false),
    }) {
        let some_ball_entity_a = balls_query.get(*entity_a);
        let some_ball_entity_b = balls_query.get(*entity_b);

        if some_ball_entity_a.is_ok() && some_ball_entity_b.is_ok() && started {
            cluster_check_cooldown.to_check.extend(vec![
                some_ball_entity_a.unwrap().0,
                some_ball_entity_b.unwrap().0,
            ]);
            if cluster_check_cooldown.timer.just_finished() {
                writer_find_cluster.send(FindCluster {
                    to_check: cluster_check_cooldown
                        .to_check
                        .clone()
                        .into_iter()
                        .collect(),
                    move_down_after: false,
                });
                cluster_check_cooldown.to_check.clear();
                println!("send FindCluster");
                println!("projectile_query len {}", projectile_query.iter().len());
            }
        }
        if let Ok((_, ball_transform)) = some_ball_entity_a.or(some_ball_entity_b) {
            let mut p1 = projectile_query.get_mut(*entity_a);
            if p1.is_err() {
                p1 = projectile_query.get_mut(*entity_b);
            }

            if let Ok((
                projectile_entity,
                projectile_transform,
                projectile_velocity,
                mut projectile_ball,
            )) = p1
            {
                println!("col {:?}", projectile_entity);
                // take into account only collision between projectile and grid ball
                if started {
                    if projectile_ball.snap_vel == Vec2::ZERO {
                        // snap with revolute joint only to the first grid ball
                        let to_pos = ball_transform.translation.truncate();
                        let from_pos = projectile_transform.translation.truncate();
                        let diff = (to_pos - from_pos).normalize();
                        let vel = projectile_velocity.linvel.normalize();
                        let dot = vel.dot(diff);
                        if dot > MIN_PROJECTILE_SNAP_DOT {
                            println!("dot > MIN_PROJECTILE_SNAP_DOT {:?}", projectile_entity);
                            collision_snap_cooldown.start();
                            // save first touch position
                            is_move_reverse(&mut projectile_ball, projectile_velocity.linvel);
                            commands
                                .entity(projectile_entity)
                                .insert(MagneticGridBall {});
                        }
                    }
                }
                let is_move_reverse_result =
                    is_move_reverse(&mut projectile_ball, projectile_velocity.linvel);
                println!(
                    "{:?} started {} is_move_slow {} is_move_reverse {}",
                    projectile_entity,
                    started,
                    is_move_slow(projectile_velocity.linvel),
                    is_move_reverse_result
                );
                if is_move_slow(projectile_velocity.linvel) || is_move_reverse_result {
                    // if ball turned back
                    // or ball moves too slow
                    info!("Projectile too slow so snap");
                    send_snap_projectile(
                        collision_snap_cooldown.as_mut(),
                        &mut writer_snap_projectile,
                        projectile_entity,
                    );
                }
            }
        }
    }
}

pub fn on_snap_projectile(
    mut snap_projectile_events: EventReader<SnapProjectile>,
    mut commands: Commands,
    grid: Res<Grid>,
    mut projectile_reload_writer: EventWriter<ProjectileReload>,
    mut turn_counter: ResMut<TurnCounter>,
    mut writer_find_cluster: EventWriter<FindCluster>,
    mut projectile_query: Query<(&mut ProjectileBall, &Transform), With<ProjectileBall>>,
) {
    // println!("on_snap_projectile");
    for SnapProjectile { projectile_entity } in snap_projectile_events.iter() {
        println!("SnapProjectile process {:?}", projectile_entity);
        if let Ok((mut projectile_ball, transform)) = projectile_query.get_mut(*projectile_entity) {
            // projectile ball can be removed by cluster and never snapped
            if projectile_ball.is_snapped {
                continue;
            }
            projectile_ball.is_snapped = true;
            let position = transform.translation.truncate();
            let snap_hex = grid.layout.world_pos_to_hex(position);
            let mut offset = snap_hex.to_offset_coordinates(grid.offset_mode);
            let mut is_last_active = false;
            let mut entity_commands = commands.entity(*projectile_entity);
            if offset[1] <= grid.last_active_row {
                offset[1] = grid.last_active_row;
                is_last_active = true;
                let corrected_position = grid
                    .layout
                    .hex_to_world_pos(Hex::from_offset_coordinates(offset, grid.offset_mode));

                entity_commands.insert(GridBallPositionAnimate::from_position(
                    corrected_position,
                    false,
                ));
            }
            if is_last_active {
                entity_commands.insert(LastActiveGridBall {});
            }
            entity_commands.remove::<ProjectileBall>();
            println!(
                "removed ProjectileBall from {:?} position y {}",
                projectile_entity, position.y
            );
        }
        turn_counter.0 += 1;

        println!("send ProjectileReload {:?}", projectile_entity);
        projectile_reload_writer.send(ProjectileReload);
        writer_find_cluster.send(FindCluster {
            to_check: vec![*projectile_entity],
            move_down_after: true,
        });
    }
}

pub fn find_and_remove_clusters(
    mut commands: Commands,
    mut find_cluster_events: EventReader<FindCluster>,
    mut balls_query: Query<
        (
            Entity,
            &Transform,
            &Species,
            &mut GridBall,
            Option<&LastActiveGridBall>,
            Option<&ProjectileBall>,
        ),
        With<GridBall>,
    >,
    gameplay_meshes: Res<GameplayMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut writer_update_cooldown_counter: EventWriter<UpdateScoreCounter>,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
) {
    if find_cluster_events.is_empty() {
        return;
    }
    // println!("FindCluster len {}", find_cluster_events.iter().len());
    let mut entities_to_positions: HashMap<Entity, Vec2> = HashMap::default();
    let mut entities_to_species: HashMap<Entity, Species> = HashMap::default();
    let mut last_active_entities: HashSet<Entity> = HashSet::default();
    balls_query.iter().for_each(|(e, t, sp, gb, ila, _)| {
        if !gb.is_ready_to_despawn {
            entities_to_positions.insert(e, t.translation.truncate());
            entities_to_species.insert(e, *sp);
            if let Some(_) = ila {
                last_active_entities.insert(e);
            }
        }
    });
    let cells_to_entities = buid_cells_to_entities(&entities_to_positions);
    let mut entities_to_neighbours =
        build_entities_to_neighbours(&entities_to_positions, &cells_to_entities);

    for FindCluster {
        to_check,
        move_down_after,
    } in find_cluster_events.iter()
    {
        println!("FindCluster iter to_check {}", to_check.len());
        for start_from in to_check.iter() {
            let (cluster, _) =
                find_cluster(*start_from, &entities_to_neighbours, &entities_to_species);

            let mut cluster_score_add = 0;
            if cluster.len() >= MIN_CLUSTER_SIZE {
                // remove matching cluster
                cluster.iter().for_each(|cluster_entity| {
                    if let Ok((
                        cluster_entity,
                        cluster_transform,
                        cluster_species,
                        mut grid_ball,
                        _,
                        some_projectile_ball,
                    )) = balls_query.get_mut(*cluster_entity)
                    {
                        if !grid_ball.is_ready_to_despawn {
                            grid_ball.is_ready_to_despawn = true;
                            commands.spawn(OutBallBundle::new(
                                cluster_transform.translation.truncate(),
                                *cluster_species,
                                &gameplay_meshes,
                                &mut materials,
                                false,
                            ));
                            println!("cluster entity despawned {:?}", cluster_entity);
                            commands.entity(cluster_entity).despawn_recursive();
                            entities_to_neighbours.remove(&cluster_entity);
                            cluster_score_add += 1;
                            if some_projectile_ball.is_some() {
                                println!("cluster entity is projectile {:?}", cluster_entity);
                                writer_snap_projectile.send(SnapProjectile {
                                    projectile_entity: cluster_entity,
                                });
                            }
                        }
                    }
                });
            }

            // let floating_clusters_score_add = 0;
            // let floating_clusters =
            //     find_floating_clusters(&cluster, &entities_to_neighbours, &last_active_entities);
            // remove floating clusters
            // floating_clusters
            //     .iter()
            //     .flat_map(|e| e.iter())
            //     .for_each(|entity| {
            //         if let Ok((cluster_entity, cluster_transform, cluster_species, mut grid_ball, _)) =
            //             balls_query.get_mut(*entity)
            //         {
            //             if !grid_ball.is_ready_to_despawn {
            //                 grid_ball.is_ready_to_despawn = true;
            //                 commands.spawn(OutBallBundle::new(
            //                     cluster_transform.translation.truncate(),
            //                     *cluster_species,
            //                     &gameplay_meshes,
            //                     &mut materials,
            //                     true,
            //                 ));
            //                 println!("floating cluster entity despawned {:?}", cluster_entity);
            //                 commands.entity(cluster_entity).despawn_recursive();
            //                 floating_clusters_score_add += 2;
            //             }
            //         }
            //     });

            // let score_add = cluster_score_add + floating_clusters_score_add;
            let score_add = cluster_score_add;

            // println!("send UpdateScoreCounter");
            writer_update_cooldown_counter.send(UpdateScoreCounter {
                score_add,
                move_down_after: *move_down_after,
            });
        }
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
    mut writer_move_down_last_active: EventWriter<MoveDownLastActive>,
) {
    if let Some(UpdateScoreCounter {
        score_add,
        move_down_after,
    }) = update_cooldown_counter_events.iter().next()
    {
        if *score_add > 0 {
            pkv_play_score_audio(&mut commands, &audio_assets, &pkv);
            score_counter.0 += score_add;
        } else if cooldown_move_counter.init_value != 0 && *move_down_after {
            cooldown_move_counter.value -= 1;
            if cooldown_move_counter.value == 0 {
                move_counter.0 += 1;
                cooldown_move_counter.value = cooldown_move_counter.init_value;
                writer_move_down_last_active.send(MoveDownLastActive {});
            }
        }
    }
    update_cooldown_counter_events.clear();
}

pub fn tick_collision_snap_cooldown_timer(
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut ProjectileBall, &Velocity), With<ProjectileBall>>,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
) {
    if !collision_snap_cooldown.timer.paused() {
        collision_snap_cooldown.timer.tick(time.delta());
        if let Ok((projectile_entity, mut projectile_ball, projectile_velocity)) =
            projectile_query.get_single_mut()
        {
            if collision_snap_cooldown.is_ready_for_check(|| {
                println!("is_ready_for_check {}", projectile_velocity.linvel.length());
                is_move_slow(projectile_velocity.linvel)
                    || is_move_reverse(&mut projectile_ball, projectile_velocity.linvel)
            }) {
                // snap projectile anyway after some time
                info!("Projectile cooldown snap");
                send_snap_projectile(
                    collision_snap_cooldown.as_mut(),
                    &mut writer_snap_projectile,
                    projectile_entity,
                );
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
            if grid_ball_animate.move_down_after {
                completed_count += 1;
            }
        }
    }
    if completed_count == total_count && completed_count > 0 {
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
            true,
            false,
            None,
            true,
            true,
        );
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
            println!("on_spawn_row despawn_recursive {:?}", ball_entity);
            commands.entity(ball_entity).despawn_recursive();
            let ball_position = ball_transform.translation.truncate();
            GridBallBundle::spawn(
                &mut commands,
                &gameplay_meshes,
                &gameplay_materials,
                ball_position,
                false,
                true,
                false,
                Some(*ball_species),
                false,
                true,
            );
        }
    }
}
