use std::collections::HashMap;

use bevy::{
    prelude::{
        Assets, Commands, DespawnRecursiveExt, Entity, EventReader, Handle, Input, Mesh,
        MouseButton, Query, Res, ResMut, StandardMaterial, Touches, Transform, Vec2, Vec3,
        Visibility, With, Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::Velocity;

use crate::{
    game_audio::utils::pkv_play_shoot_audio,
    gameplay::{
        constants::{PROJECTILE_SHOOT, PROJECTILE_SPAWN},
        events::BeginTurn,
        grid::resources::Grid,
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
    },
    loading::audio_assets::AudioAssets,
    ui::resources::PointerCooldown,
};

use super::{
    aim_bundle::AimBundle,
    components::{AimLine, AimTarget, GridBall, OutBall, ProjectileBall, Species},
    constants::{INNER_RADIUS_COEFF, PROJECTILE_SPEED},
    projectile_ball_bundle::ProjectileBallBundle,
    resources::ProjectileBuffer,
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

    commands.spawn(ProjectileBallBundle::new(
        Vec2::new(0.0, PROJECTILE_SPAWN),
        grid.layout.hex_size.x,
        species,
        &gameplay_meshes,
        &gameplay_materials,
    ));

    buffer.0.push(Species::pick_random(&colors_in_grid));
}

pub fn shoot_projectile(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut projectile_ball_query: Query<
        (&Transform, &mut Velocity, &mut ProjectileBall),
        (With<ProjectileBall>, Without<AimTarget>, Without<AimLine>),
    >,
    mouse_button_input: Res<Input<MouseButton>>,
    audio_assets: Res<AudioAssets>,
    mut aim_target_query: Query<
        (&mut Transform, &mut Visibility),
        (With<AimTarget>, Without<ProjectileBall>, Without<AimLine>),
    >,
    mut aim_line_query: Query<
        (&mut Transform, &mut Visibility),
        (With<AimLine>, Without<ProjectileBall>, Without<AimTarget>),
    >,
    touches: Res<Touches>,
    pointer_cooldown: Res<PointerCooldown>,
    pkv: Res<PkvStore>,
) {
    if pointer_cooldown.started {
        return;
    }
    let window = window_query.single();
    let mut pointer_position = Vec2::ZERO;

    if mouse_button_input.pressed(MouseButton::Left)
        || mouse_button_input.just_released(MouseButton::Left)
    {
        if let Some(cursor_position) = window.cursor_position() {
            pointer_position = Vec2::new(cursor_position.x, window.height() - cursor_position.y)
        }
    }
    if let Some(touch) = touches.iter().next() {
        let touch_position = touch.position();
        pointer_position = Vec2::new(touch_position.x, window.height() - touch_position.y);
    } else if let Some(touch) = touches.iter_just_released().next() {
        let touch_position = touch.position();
        pointer_position = Vec2::new(touch_position.x, window.height() - touch_position.y);
    }
    if let Ok((mut target_transform, mut target_visibility)) = aim_target_query.get_single_mut() {
        if let Ok((mut line_transform, mut line_visibility)) = aim_line_query.get_single_mut() {
            if let Ok((ball_transform, mut vel, mut projectile_ball)) =
                projectile_ball_query.get_single_mut()
            {
                if pointer_position.length() > Vec2::ZERO.length() && !projectile_ball.is_flying {
                    *target_visibility = Visibility::Visible;
                    *line_visibility = Visibility::Visible;

                    let mut projectile_position = pointer_position;
                    if projectile_position.y > PROJECTILE_SHOOT {
                        projectile_position.y = PROJECTILE_SHOOT;
                    }

                    target_transform.translation.x = projectile_position.x;
                    target_transform.translation.y = projectile_position.y;

                    // line_parent_transform.translation.x =
                    //     (projectile_position.x - ball_transform.translation.x) / 2.0;
                    // line_parent_transform.translation.y = projectile_position.y
                    //     + (ball_transform.translation.y - projectile_position.y) / 2.0;

                    line_transform.scale.y = ball_transform
                        .translation
                        .distance(target_transform.translation)
                        - INNER_RADIUS_COEFF * 2.0;
                    // line_parent_transform.look_at(
                    //     Vec3::new(projectile_position.x, projectile_position.y, 0.0),
                    //     Vec3::Z,
                    // );

                    if !(mouse_button_input.just_released(MouseButton::Left)
                        || touches.any_just_released())
                    {
                        return;
                    }

                    let aim_direction =
                        (projectile_position - ball_transform.translation.truncate()).normalize();
                    vel.linvel = aim_direction * PROJECTILE_SPEED;

                    projectile_ball.is_flying = true;

                    pkv_play_shoot_audio(&mut commands, &audio_assets, &pkv);
                } else {
                    *target_visibility = Visibility::Hidden;
                    *line_visibility = Visibility::Hidden;
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
        Vec2::new(0.0, PROJECTILE_SPAWN / 2.0),
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

pub fn setup_aim_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
) {
    commands.spawn(AimBundle::new_line(
        Vec2::new(0.0, PROJECTILE_SPAWN / 2.0),
        &mut meshes,
        &gameplay_materials,
    ));
}

pub fn cleanup_aim_line(mut commands: Commands, aim_line_query: Query<Entity, With<AimLine>>) {
    for projectile_entity in aim_line_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

pub fn animate_out_ball(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut balls_query: Query<
        (&mut OutBall, &mut Transform, &Handle<StandardMaterial>),
        With<OutBall>,
    >,
) {
    for (mut grid_ball_out, mut ball_transform, ball_material) in balls_query.iter_mut() {
        if !grid_ball_out.started {
            grid_ball_out.started = true;
            grid_ball_out.initial_velocity =
                Vec3::new(-0.2 + fastrand::f32() * 0.4, fastrand::f32() * 0.5, 0.0);
        } else {
            grid_ball_out.initial_velocity += Vec3::new(0.0, 0.0, 0.01); // gravity
            ball_transform.translation += grid_ball_out.initial_velocity;
            if let Some(ball_material) = materials.get_mut(&ball_material) {
                ball_material
                    .base_color
                    .set_a(ball_material.base_color.a() - 0.01);
            }
        }
    }
}

pub fn check_out_ball_bounds(
    mut commands: Commands,
    out_balls_query: Query<(Entity, &OutBall, &Transform), With<OutBall>>,
) {
    for (ball_entity, grid_ball_out, ball_transform) in out_balls_query.iter() {
        if grid_ball_out.started && ball_transform.translation.z > PROJECTILE_SPAWN {
            commands.entity(ball_entity).despawn_recursive();
        }
    }
}
