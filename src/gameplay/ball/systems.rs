use std::ops::Add;

use bevy::{
    prelude::{
        warn, Assets, Camera, ColorMaterial, Commands, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, Handle, Input, Mesh, MouseButton, Query, Res, ResMut, Touches, Transform,
        Vec2, Visibility, With, Without,
    },
    time::Time,
    utils::HashSet,
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_xpbd_2d::prelude::{
    Collider, ExternalForce, LinearVelocity, Position, ShapeHitData, SpatialQuery,
    SpatialQueryFilter,
};

use crate::{
    game_audio::utils::pkv_play_shoot_audio,
    gameplay::{
        constants::{
            APPEAR_TOLERANCE, BALL_RADIUS, CAST_RAY_BOUNCE_Y_ADD, CAST_RAY_MAX_TOI, CAST_RAY_TRIES,
            CAST_RAY_VELOCITY, CAST_RAY_VELOCITY_TOLERANCE, NEXT_PROJECTILE_SPAWN_BOTTOM,
            NEXT_PROJECTILE_SPAWN_SIDE, OUT_BALL_GRAVITY, PROJECTILE_SHOOT_BOTTOM,
            PROJECTILE_SPAWN_BOTTOM, PROJECTILE_SPEED,
        },
        events::ProjectileReload,
        grid::resources::Grid,
        lines::components::LineType,
        main_camera::components::MainCamera,
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        physics::layers::Layer,
        utils::detect_pointer_position,
        walls::components::WallType,
    },
    loading::audio_assets::AudioAssets,
    ui::resources::PointerCooldown,
};

use super::{
    aim_bundle::AimBundle,
    components::{
        AimLine, AimTarget, GridBall, GridBallScaleAnimate, NextProjectileBall, OutBall,
        OutBallAnimation, ProjectileBall, Species,
    },
    grid_ball_bundle::GridBallBundle,
    projectile_ball_bundle::NextProjectileBallBundle,
    resources::ProjectileBuffer,
    utils::{cleanup_aim_line_utils, cleanup_next_projectile_ball_utils},
};

pub fn cleanup_projectile_ball(
    mut commands: Commands,
    projectile_query: Query<Entity, With<ProjectileBall>>,
) {
    for projectile_entity in projectile_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

pub fn projectile_reload(
    mut commands: Commands,
    grid: Res<Grid>,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut buffer: ResMut<ProjectileBuffer>,
    mut projectile_reload_events: EventReader<ProjectileReload>,
    grid_balls_query: Query<&Species, (With<GridBall>, Without<ProjectileBall>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    next_projectile_query: Query<Entity, With<NextProjectileBall>>,
    projectile_query: Query<Entity, With<ProjectileBall>>,
) {
    if grid_balls_query.iter().len() == 0 {
        for projectile_entity in projectile_query.iter() {
            commands.entity(projectile_entity).despawn_recursive();
        }
        return; // no more balls in grid
    }

    if projectile_reload_events.is_empty() {
        return;
    }
    projectile_reload_events.clear();
    println!("received ProjectileReload");

    let mut cache: HashSet<&Species> = HashSet::with_capacity(5);
    for species in grid_balls_query.iter() {
        if cache.len() == 5 {
            break;
        }
        if let None = cache.get(species) {
            cache.insert(species);
        }
    }

    let mut colors_in_grid: Vec<Species> = Vec::with_capacity(cache.len());
    for key in cache.iter() {
        colors_in_grid.push(**key);
    }

    let species = match buffer.0.pop() {
        Some(species) => {
            // if picked from buffer color is absent in grid
            // generate the new one
            if let Some(_) = colors_in_grid
                .iter()
                .find(|grid_species| **grid_species == species)
            {
                species
            } else {
                Species::pick_random(&colors_in_grid, grid.total_colors)
            }
        }
        None => Species::pick_random(&colors_in_grid, grid.total_colors),
    };

    let window = window_query.single();
    let projectile_spawn_bottom =
        -(window.height() - PROJECTILE_SPAWN_BOTTOM - window.height() / 2.0);

    let (entity, _) = GridBallBundle::spawn(
        &mut commands,
        &gameplay_meshes,
        &gameplay_materials,
        grid.total_colors,
        Vec2::new(0.0, projectile_spawn_bottom),
        false,
        true,
        Some(species),
        true,
        true,
    );
    println!(
        "=>>>>> ProjectileReload spawn {:?} {} y {}",
        entity, species, projectile_spawn_bottom
    );

    buffer
        .0
        .push(Species::pick_random(&colors_in_grid, grid.total_colors));

    cleanup_next_projectile_ball_utils(&mut commands, &next_projectile_query);
    if let Some(species) = buffer.0.last() {
        let next_projectile_spawn_bottom: f32 =
            -(window.height() - NEXT_PROJECTILE_SPAWN_BOTTOM - window.height() / 2.0);

        commands.spawn(NextProjectileBallBundle::new(
            Vec2::new(-NEXT_PROJECTILE_SPAWN_SIDE, next_projectile_spawn_bottom),
            *species,
            &gameplay_meshes,
            &gameplay_materials,
        ));

        commands.spawn(NextProjectileBallBundle::new(
            Vec2::new(NEXT_PROJECTILE_SPAWN_SIDE, next_projectile_spawn_bottom),
            *species,
            &gameplay_meshes,
            &gameplay_materials,
        ));
    }
}

pub fn shoot_projectile(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    mut projectile_ball_query: Query<
        (Entity, &Position, &mut LinearVelocity, &mut ProjectileBall),
        (With<ProjectileBall>, Without<AimTarget>, Without<AimLine>),
    >,
    audio_assets: Res<AudioAssets>,
    pointer_cooldown: Res<PointerCooldown>,
    pkv: Res<PkvStore>,
    mut aim_target_query: Query<&mut AimTarget, With<AimTarget>>,
) {
    if pointer_cooldown.started {
        return;
    }
    let (pointer_position, pointer_pressed, pointer_released, pointer_aquired) =
        detect_pointer_position(&window_query, &camera_query, &mouse_button_input, &touches);

    if !(pointer_pressed || pointer_released) {
        return;
    }

    for (projectile_entity, projectile_position, mut linear_velocity, mut projectile_ball) in
        projectile_ball_query.iter_mut()
    {
        if projectile_ball.is_flying {
            continue;
        }
        let window = window_query.single();
        let projectile_shoot_bottom =
            -(window.height() - PROJECTILE_SHOOT_BOTTOM - window.height() / 2.0);

        let mut target_position = pointer_position;
        if target_position.y < projectile_shoot_bottom {
            target_position.y = projectile_shoot_bottom;
        }

        if let Ok(mut aim) = aim_target_query.get_single_mut() {
            aim.pointer_pressed = pointer_pressed;
            aim.pointer_released = pointer_released;
            if pointer_pressed {
                if projectile_ball.is_flying {
                    aim.pointer_pressed = false
                } else {
                    aim.aim_pos = projectile_position.0;
                    aim.aim_vel = target_position - aim.aim_pos;
                }
            }
        }

        if pointer_aquired && !projectile_ball.is_flying && pointer_released {
            let aim_direction = target_position - projectile_position.0;
            linear_velocity.0 = aim_direction.normalize() * PROJECTILE_SPEED;

            println!(
                "SHOOOOOT {:?} linear_velocity {} position {}",
                projectile_entity, linear_velocity.0, projectile_position.0
            );
            projectile_ball.is_flying = true;

            pkv_play_shoot_audio(&mut commands, &audio_assets, &pkv);
        }
    }
}

pub fn draw_aim(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
    wall_query: Query<(Entity, &WallType), With<WallType>>,
    balls_query: Query<Entity, With<GridBall>>,
    spatial_query: SpatialQuery,
    aim_line_query: Query<Entity, With<AimLine>>,
    mut aim_target_query: Query<
        (&mut AimTarget, &mut Transform, &mut Visibility),
        (With<AimTarget>, Without<LineType>),
    >,
    projectile_ball_query: Query<Entity, With<ProjectileBall>>,
) {
    if let Ok((mut aim_target, mut target_transform, mut target_visibility)) =
        aim_target_query.get_single_mut()
    {
        if aim_target.pointer_released {
            *target_visibility = Visibility::Hidden;
            cleanup_aim_line_utils(&mut commands, &aim_line_query);
            aim_target.draw_vel = Vec2::ZERO;
        }

        if aim_target.pointer_pressed && aim_target.draw_vel != aim_target.aim_vel {
            aim_target.draw_vel = aim_target.aim_vel;
            // redraw only if pointer position (draw velocity in this case) changed

            cleanup_aim_line_utils(&mut commands, &aim_line_query);
            let mut exclude_entities: HashSet<Entity> = HashSet::default();
            for projectile_entity in projectile_ball_query.iter() {
                exclude_entities.insert(projectile_entity);
            }
            let spatial_query_filter = SpatialQueryFilter::default()
                .with_masks([Layer::Walls, Layer::Grid])
                .without_entities(exclude_entities);

            let shape = Collider::ball(BALL_RADIUS);
            let mut ray_start = aim_target.aim_pos;
            let mut ray_vel = aim_target.aim_vel.normalize() * CAST_RAY_VELOCITY;
            let mut count = 0;

            loop {
                count = count + 1;
                if count > CAST_RAY_TRIES {
                    let message = format!("Break aim iteration draw, reached max iteration count aim.aim_pos({:?}) aim_vel({:?})", aim_target.aim_pos, aim_target.aim_vel);
                    warn!(message);
                    break;
                }
                if let Some(ShapeHitData {
                    entity,
                    time_of_impact: _,
                    point1,
                    point2,
                    normal1: _,
                    normal2: _,
                }) = spatial_query.cast_shape(
                    &shape,
                    ray_start,
                    0.0,
                    ray_vel,
                    CAST_RAY_MAX_TOI,
                    true,
                    spatial_query_filter.clone(),
                ) {
                    if let Ok((_, wall_type)) = wall_query.get(entity) {
                        let mut center = point1 - point2;
                        if wall_type.is_side() {
                            center = center.add(Vec2::new(
                                0.0,
                                CAST_RAY_BOUNCE_Y_ADD * ray_vel.normalize().y,
                            ));
                        } else {
                            target_transform.translation =
                                center.extend(target_transform.translation.z);
                            *target_visibility = Visibility::Visible;
                        }
                        commands.spawn(AimBundle::new_line(
                            ray_start,
                            center,
                            &mut meshes,
                            &gameplay_materials,
                        ));
                        if wall_type.is_top() {
                            break;
                        }
                        ray_start = center;
                        let mut reverse_x_vel = -ray_vel.x;
                        match reverse_x_vel.signum() > 0.0 {
                            true => reverse_x_vel += CAST_RAY_VELOCITY_TOLERANCE,
                            false => reverse_x_vel -= CAST_RAY_VELOCITY_TOLERANCE,
                        }
                        ray_vel = Vec2::new(reverse_x_vel, ray_vel.y);
                    } else if let Ok(_) = balls_query.get(entity) {
                        let center = point1 - point2;
                        target_transform.translation =
                            center.extend(target_transform.translation.z);
                        commands.spawn(AimBundle::new_line(
                            ray_start,
                            center,
                            &mut meshes,
                            &gameplay_materials,
                        ));
                        *target_visibility = Visibility::Visible;
                        break;
                    }
                }
            }
        }
    }
}

pub fn setup_aim_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
    grid: Res<Grid>,
) {
    commands.spawn(AimBundle::new_target(
        Vec2::new(0.0, 0.0),
        &mut meshes,
        &gameplay_materials,
        &grid,
    ));
}

pub fn cleanup_aim_target(
    mut commands: Commands,
    aim_target_query: Query<Entity, With<AimTarget>>,
) {
    for projectile_entity in aim_target_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

pub fn cleanup_aim_lines(mut commands: Commands, aim_line_query: Query<Entity, With<AimLine>>) {
    cleanup_aim_line_utils(&mut commands, &aim_line_query);
}

pub fn animate_out_ball(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut balls_query: Query<
        (
            &mut OutBall,
            &mut Transform,
            &mut LinearVelocity,
            &mut ExternalForce,
            &Handle<ColorMaterial>,
        ),
        With<OutBall>,
    >,
) {
    for (
        mut grid_ball_out,
        mut ball_transform,
        mut linear_velocity,
        mut external_force,
        ball_material,
    ) in balls_query.iter_mut()
    {
        if !grid_ball_out.started {
            grid_ball_out.started = true;
            ball_transform.translation.z = 2.0; // slightly on top of grid
            if grid_ball_out.animation_type == OutBallAnimation::FloatingCluster {
                linear_velocity.0 = Vec2::new(0.0, fastrand::i32(-200..=0) as f32);
            } else {
                linear_velocity.0 = Vec2::new(
                    match fastrand::bool() {
                        true => fastrand::i32(-200..=-100) as f32,
                        false => fastrand::i32(100..=200) as f32,
                    },
                    fastrand::i32(100..=200) as f32,
                );
            }
            external_force.set_force(Vec2::new(0.0, -OUT_BALL_GRAVITY));
        } else {
            if let Some(ball_material) = materials.get_mut(&ball_material) {
                ball_material.color.set_a(ball_material.color.a() - 0.01);
                if ball_material.color.a() <= 0.0 {
                    grid_ball_out.marked_for_delete = true;
                }
            }
        }
    }
}

pub fn check_out_ball_for_delete(
    mut commands: Commands,
    out_balls_query: Query<(Entity, &OutBall), With<OutBall>>,
) {
    for (ball_entity, grid_ball_out) in out_balls_query.iter() {
        if grid_ball_out.marked_for_delete {
            commands.entity(ball_entity).despawn_recursive();
        }
    }
}

pub fn cleanup_next_projectile_ball(
    mut commands: Commands,
    query: Query<Entity, With<NextProjectileBall>>,
) {
    cleanup_next_projectile_ball_utils(&mut commands, &query)
}

pub fn animate_grid_ball_scale(
    mut commands: Commands,
    mut grid_balls_query: Query<
        (Entity, &mut Transform, &mut GridBallScaleAnimate),
        With<GridBallScaleAnimate>,
    >,
    time: Res<Time>,
) {
    for (ball_entity, mut grid_ball_transform, mut grid_ball_animate) in grid_balls_query.iter_mut()
    {
        grid_ball_animate.timer.tick(time.delta());
        grid_ball_transform.scale = grid_ball_transform
            .scale
            .truncate()
            .lerp(grid_ball_animate.scale, grid_ball_animate.timer.percent())
            .extend(grid_ball_transform.scale.z);
        if (grid_ball_transform.scale.truncate() - grid_ball_animate.scale).length()
            < APPEAR_TOLERANCE
        {
            grid_ball_transform.scale = grid_ball_animate.scale.extend(grid_ball_transform.scale.z);
            commands
                .entity(ball_entity)
                .remove::<GridBallScaleAnimate>();
        }
    }
}
