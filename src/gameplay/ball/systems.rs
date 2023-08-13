use std::{collections::HashMap, ops::Add};

use bevy::{
    prelude::{
        warn, Assets, Camera, ColorMaterial, Commands, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, Handle, Input, Mesh, MouseButton, Query, Res, ResMut, Touches, Transform,
        Vec2, Visibility, With, Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, ExternalForce, Group, QueryFilter, RapierContext, Velocity,
};

use crate::{
    game_audio::utils::pkv_play_shoot_audio,
    gameplay::{
        constants::{
            BALL_RADIUS, CAST_RAY_BOUNCE_Y_ADD, CAST_RAY_MAX_TOI, CAST_RAY_TRIES,
            CAST_RAY_VELOCITY, CAST_RAY_VELOCITY_TOLERANCE, NEXT_PROJECTILE_SPAWN_BOTTOM,
            NEXT_PROJECTILE_SPAWN_SIDE, PROJECTILE_SHOOT_BOTTOM, PROJECTILE_SPAWN_BOTTOM,
            PROJECTILE_SPEED,
        },
        events::BeginTurn,
        grid::resources::Grid,
        lines::components::LineType,
        main_camera::components::MainCamera,
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        utils::detect_pointer_position,
        walls::components::WallType,
    },
    loading::audio_assets::AudioAssets,
    ui::resources::PointerCooldown,
};

use super::{
    aim_bundle::AimBundle,
    components::{
        AimLine, AimTarget, GridBall, NextProjectileBall, OutBall, ProjectileBall, Species,
    },
    projectile_ball_bundle::{NextProjectileBallBundle, ProjectileBallBundle},
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
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut buffer: ResMut<ProjectileBuffer>,
    mut begin_turn: EventReader<BeginTurn>,
    grid: Res<Grid>,
    balls_query: Query<&Species, With<GridBall>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    next_projectile_query: Query<Entity, With<NextProjectileBall>>,
) {
    if begin_turn.is_empty() {
        return;
    }
    begin_turn.clear();

    let mut cache: HashMap<&Species, &Species> = HashMap::with_capacity(5);
    for species in balls_query.iter() {
        if cache.len() == 5 {
            break;
        }
        if let None = cache.get(species) {
            cache.insert(species, species);
        }
    }
    if grid.storage.len() == 0 {
        return; // no more balls in grid
    }
    let mut colors_in_grid: Vec<Species> = Vec::with_capacity(cache.len());
    for (key, _) in cache.iter() {
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
                Species::pick_random(&colors_in_grid)
            }
        }
        None => Species::pick_random(&colors_in_grid),
    };

    let window = window_query.single();
    let projectile_spawn_bottom =
        -(window.height() - PROJECTILE_SPAWN_BOTTOM - window.height() / 2.0);

    commands.spawn(ProjectileBallBundle::new(
        Vec2::new(0.0, projectile_spawn_bottom),
        species,
        &gameplay_meshes,
        &gameplay_materials,
    ));

    buffer.0.push(Species::pick_random(&colors_in_grid));

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
        (&Transform, &mut Velocity, &mut ProjectileBall),
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

    if let Ok((projectile_transform, mut vel, mut projectile_ball)) =
        projectile_ball_query.get_single_mut()
    {
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
                    aim.aim_pos = projectile_transform.translation.truncate();
                    aim.aim_vel = target_position - aim.aim_pos;
                }
            }
        }

        if pointer_aquired && !projectile_ball.is_flying && pointer_released {
            let aim_direction = target_position - projectile_transform.translation.truncate();
            vel.linvel = aim_direction.normalize() * PROJECTILE_SPEED;

            projectile_ball.is_flying = true;

            pkv_play_shoot_audio(&mut commands, &audio_assets, &pkv);
        }
    }
}

pub fn draw_aim(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
    wall_query: Query<Entity, With<WallType>>,
    balls_query: Query<Entity, With<GridBall>>,
    rapier_context: Res<RapierContext>,
    aim_line_query: Query<Entity, With<AimLine>>,
    lines_query: Query<(&LineType, &Transform), With<LineType>>,
    mut aim_target_query: Query<
        (&mut AimTarget, &mut Transform, &mut Visibility),
        (With<AimTarget>, Without<LineType>),
    >,
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
            let kinematic_filter = QueryFilter::default().groups(CollisionGroups::new(
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_5,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_5,
            ));
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
                if let Some((entity, hit)) = rapier_context.cast_shape(
                    ray_start,
                    0.0,
                    ray_vel,
                    &shape,
                    CAST_RAY_MAX_TOI,
                    kinematic_filter,
                ) {
                    if let Ok(_) = wall_query.get(entity) {
                        let center = (hit.witness1 - hit.witness2).add(Vec2::new(
                            0.0,
                            CAST_RAY_BOUNCE_Y_ADD * ray_vel.normalize().y,
                        ));
                        commands.spawn(AimBundle::new_line(
                            ray_start,
                            center,
                            &mut meshes,
                            &gameplay_materials,
                        ));
                        ray_start = center;
                        let mut reverse_x_vel = -ray_vel.x;
                        match reverse_x_vel.signum() > 0.0 {
                            true => reverse_x_vel += CAST_RAY_VELOCITY_TOLERANCE,
                            false => reverse_x_vel -= CAST_RAY_VELOCITY_TOLERANCE,
                        }
                        ray_vel = Vec2::new(reverse_x_vel, ray_vel.y);
                    } else if let Ok(_) = balls_query.get(entity) {
                        let center = hit.witness1 - hit.witness2;
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
                    } else if let Ok(_) = lines_query.get(entity) {
                        let center = hit.witness1 - hit.witness2;
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
            &mut Velocity,
            &mut ExternalForce,
            &Handle<ColorMaterial>,
        ),
        With<OutBall>,
    >,
) {
    for (mut grid_ball_out, mut ball_transform, mut ball_velocity, mut ball_force, ball_material) in
        balls_query.iter_mut()
    {
        if !grid_ball_out.started {
            grid_ball_out.started = true;
            ball_transform.translation.z = 2.0; // slightly on top of grid
            ball_velocity.linvel = Vec2::new(
                fastrand::i32(-200..=200) as f32,
                fastrand::i32(-200..=200) as f32,
            );
            ball_force.force = Vec2::new(0.0, -100.0);
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
            // println!("out ball despawned {}", ball_entity.index());
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
