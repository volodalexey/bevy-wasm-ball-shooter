use std::collections::HashMap;

use bevy::{
    prelude::{
        Assets, Camera, ColorMaterial, Commands, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, Handle, Input, Mesh, MouseButton, Quat, Query, Res, ResMut, Touches,
        Transform, Vec2, Visibility, With, Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::{ExternalForce, Velocity};

use crate::{
    game_audio::utils::pkv_play_shoot_audio,
    gameplay::{
        constants::{PROJECTILE_SHOOT, PROJECTILE_SPAWN},
        events::BeginTurn,
        grid::resources::Grid,
        main_camera::components::MainCamera,
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        utils::detect_pointer_position,
    },
    loading::audio_assets::AudioAssets,
    ui::resources::PointerCooldown,
    utils::from_grid_2d_to_2d,
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
        from_grid_2d_to_2d(Vec2::new(0.0, PROJECTILE_SPAWN)),
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
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    mut projectile_ball_query: Query<
        (&Transform, &mut Velocity, &mut ProjectileBall),
        (With<ProjectileBall>, Without<AimTarget>, Without<AimLine>),
    >,
    audio_assets: Res<AudioAssets>,
    mut aim_target_query: Query<
        (&mut Transform, &mut Visibility),
        (With<AimTarget>, Without<ProjectileBall>, Without<AimLine>),
    >,
    mut aim_line_query: Query<
        (&mut Transform, &mut Visibility),
        (With<AimLine>, Without<ProjectileBall>, Without<AimTarget>),
    >,
    pointer_cooldown: Res<PointerCooldown>,
    pkv: Res<PkvStore>,
) {
    if pointer_cooldown.started {
        return;
    }
    let (pointer_position, pointer_aquired) =
        detect_pointer_position(&window_query, &camera_query, &mouse_button_input, &touches);

    if let Ok((mut target_transform, mut target_visibility)) = aim_target_query.get_single_mut() {
        if let Ok((mut line_transform, mut line_visibility)) = aim_line_query.get_single_mut() {
            if let Ok((projectile_transform, mut vel, mut projectile_ball)) =
                projectile_ball_query.get_single_mut()
            {
                if pointer_aquired && !projectile_ball.is_flying {
                    *target_visibility = Visibility::Visible;
                    *line_visibility = Visibility::Visible;

                    let mut target_position = pointer_position;
                    if target_position.y < -PROJECTILE_SHOOT {
                        target_position.y = -PROJECTILE_SHOOT;
                    }

                    target_transform.translation.x = target_position.x;
                    target_transform.translation.y = target_position.y;

                    line_transform.translation.x =
                        (target_position.x - projectile_transform.translation.x) / 2.0;
                    line_transform.translation.y = target_position.y
                        + (projectile_transform.translation.y - target_position.y) / 2.0;

                    let distance = projectile_transform
                        .translation
                        .distance(target_transform.translation);
                    // println!("distance {}", distance);
                    line_transform.scale.y = distance - INNER_RADIUS_COEFF * 2.0;
                    let diff = target_transform.translation - line_transform.translation;
                    // println!("diff({}, {})", diff.x, diff.y);
                    let angle = diff.y.atan2(diff.x);
                    // println!("angle {}", angle);
                    line_transform.rotation =
                        Quat::from_rotation_z(angle + core::f32::consts::PI / 2.0);

                    if !(mouse_button_input.just_released(MouseButton::Left)
                        || touches.any_just_released())
                    {
                        return;
                    }

                    let aim_direction =
                        target_position - projectile_transform.translation.truncate();
                    vel.linvel = aim_direction.normalize() * PROJECTILE_SPEED;
                    // println!("aim_direction ({}, {})", aim_direction.x, aim_direction.y);

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
        from_grid_2d_to_2d(Vec2::new(0.0, PROJECTILE_SPAWN / 2.0)),
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
        from_grid_2d_to_2d(Vec2::new(0.0, PROJECTILE_SPAWN / 2.0)),
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
            ball_transform.translation.z = 1.0; // slightly on top of grid
            ball_velocity.linvel = Vec2::new(
                fastrand::i32(-200..=200) as f32,
                fastrand::i32(-200..=200) as f32,
            );
            ball_force.force = Vec2::new(0.0, -100.0);
        } else {
            if let Some(ball_material) = materials.get_mut(&ball_material) {
                ball_material.color.set_a(ball_material.color.a() - 0.005);
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
