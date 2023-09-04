use bevy::{
    prelude::{
        Assets, Camera, ColorMaterial, Commands, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, Handle, Input, MouseButton, Query, Res, ResMut, Touches, Vec2, With,
        Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_xpbd_2d::prelude::{LinearVelocity, Position};

use crate::{
    game_audio::utils::pkv_play_shoot_audio,
    gameplay::{
        ball::{
            components::{
                AimLine, AimTarget, GridBall, NextProjectileBall, ProjectileBall, Species,
            },
            grid_ball_bundle::GridBallBundle,
            projectile_ball_bundle::NextProjectileBallBundle,
            resources::ProjectileHelper,
            utils::cleanup_next_projectile_ball_utils,
        },
        constants::{
            NEXT_PROJECTILE_SPAWN_BOTTOM, NEXT_PROJECTILE_SPAWN_SIDE, PROJECTILE_SHOOT_BOTTOM,
            PROJECTILE_SPAWN_BOTTOM, PROJECTILE_SPEED,
        },
        events::ProjectileReload,
        grid::resources::Grid,
        main_camera::components::MainCamera,
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        utils::detect_pointer_position,
    },
    loading::audio_assets::AudioAssets,
    ui::resources::PointerCooldown,
};

pub fn cleanup_projectile_ball(
    mut commands: Commands,
    projectile_query: Query<Entity, With<ProjectileBall>>,
) {
    for projectile_entity in projectile_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

pub fn cleanup_next_projectile_ball(
    mut commands: Commands,
    query: Query<Entity, With<NextProjectileBall>>,
) {
    cleanup_next_projectile_ball_utils(&mut commands, &query)
}

pub fn projectile_reload(
    mut commands: Commands,
    grid: Res<Grid>,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut projectile_helper: ResMut<ProjectileHelper>,
    mut projectile_reload_events: EventReader<ProjectileReload>,
    grid_balls_query: Query<Entity, (With<GridBall>, Without<ProjectileBall>)>,
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

    let species = match projectile_helper.buffer.pop() {
        Some(species) => {
            // if picked from buffer color is absent in grid
            // generate the new one
            if let Some(_) = grid.active_species.get(&species) {
                species
            } else {
                Species::pick_random(&grid.active_species, grid.total_colors)
            }
        }
        None => Species::pick_random(&grid.active_species, grid.total_colors),
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

    projectile_helper.buffer.push(Species::pick_random(
        &grid.active_species,
        grid.total_colors,
    ));

    cleanup_next_projectile_ball_utils(&mut commands, &next_projectile_query);
    if let Some(species) = projectile_helper.buffer.last() {
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

pub fn check_projectile_species(
    mut projectile_query: Query<
        (&ProjectileBall, &mut Species, &Handle<ColorMaterial>),
        With<ProjectileBall>,
    >,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (projectile_ball, mut projectile_species, handle_projectile_material) in
        projectile_query.iter_mut()
    {
        if !projectile_ball.is_flying && grid.active_species.len() > 0 {
            if let None = grid.active_species.get(projectile_species.as_ref()) {
                if let Some(projectile_material) = materials.get_mut(&handle_projectile_material) {
                    let new_species = Species::pick_random(&grid.active_species, grid.total_colors);
                    println!(
                        "Change projectile color from {} into {}",
                        projectile_species.as_ref(),
                        new_species
                    );
                    *projectile_species = new_species;
                    // change material
                    projectile_material.color = new_species.into();
                }
            }
        }
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
