use std::collections::HashMap;

use bevy::{
    prelude::{
        Assets, BuildChildren, Camera, Commands, DespawnRecursiveExt, Entity, EventReader,
        GlobalTransform, Handle, Input, Mesh, MouseButton, Query, Res, ResMut, StandardMaterial,
        Touches, Transform, Vec2, Vec3, Visibility, With, Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_rapier3d::prelude::Velocity;

use crate::{
    game_audio::utils::pkv_play_shoot_audio,
    gameplay::{
        constants::PLAYER_SPAWN_Z,
        events::BeginTurn,
        grid::resources::Grid,
        main_camera::components::MainCamera,
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        utils::{plane_intersection, ray_from_mouse_position},
    },
    loading::audio_assets::AudioAssets,
    ui::resources::PointerCooldown,
};

use super::{
    components::{
        GridBall, OutBall, ProjectileArrow, ProjectileBall, ProjectileLine, ProjectileLineParent,
        Species,
    },
    constants::{INNER_RADIUS_COEFF, PROJECTILE_SPEED},
    projectile_arrow_bundle::ProjectileArrowBundle,
    projectile_ball_bundle::ProjectileBallBundle,
    projectile_line_bundle::ProjectileLineBundle,
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
        Vec3::new(0.0, 0.0, PLAYER_SPAWN_Z),
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
    mut projectile_ball_query: Query<
        (&Transform, &mut Velocity, &mut ProjectileBall),
        (
            With<ProjectileBall>,
            Without<ProjectileArrow>,
            Without<ProjectileLine>,
        ),
    >,
    mouse_button_input: Res<Input<MouseButton>>,
    audio_assets: Res<AudioAssets>,
    mut projectile_arrow_query: Query<
        (&mut Transform, &mut Visibility),
        (
            With<ProjectileArrow>,
            Without<ProjectileBall>,
            Without<ProjectileLineParent>,
            Without<ProjectileLine>,
        ),
    >,
    mut projectile_line_parent_query: Query<
        (&mut Transform, &mut Visibility),
        (
            With<ProjectileLineParent>,
            Without<ProjectileBall>,
            Without<ProjectileArrow>,
            Without<ProjectileLine>,
        ),
    >,
    mut projectile_line_query: Query<
        &mut Transform,
        (
            With<ProjectileLine>,
            Without<ProjectileBall>,
            Without<ProjectileArrow>,
            Without<ProjectileLineParent>,
        ),
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
    if let Ok((mut arrow_transform, mut arrow_visibility)) = projectile_arrow_query.get_single_mut()
    {
        if let Ok((mut line_parent_transform, mut line_visibility)) =
            projectile_line_parent_query.get_single_mut()
        {
            if let Ok(mut line_transform) = projectile_line_query.get_single_mut() {
                if let Ok((ball_transform, mut vel, mut projectile_ball)) =
                    projectile_ball_query.get_single_mut()
                {
                    if pointer_position.length() > Vec2::ZERO.length() && !projectile_ball.is_flying
                    {
                        *arrow_visibility = Visibility::Visible;
                        *line_visibility = Visibility::Visible;

                        let (camera, camera_transform) = camera_query.single();

                        let (ray_pos, ray_dir) = ray_from_mouse_position(
                            window.width(),
                            window.height(),
                            pointer_position,
                            camera,
                            camera_transform,
                        );
                        let (plane_pos, plane_normal) =
                            (Vec3::new(0., ball_transform.translation.y, 0.), Vec3::Y);

                        let mut point =
                            plane_intersection(ray_pos, ray_dir, plane_pos, plane_normal);
                        point.y = 0.0;
                        if point.z > PLAYER_SPAWN_Z - 2.0 {
                            point.z = PLAYER_SPAWN_Z - 2.0;
                        }

                        arrow_transform.translation = point;

                        line_parent_transform.translation.x =
                            (point.x - ball_transform.translation.x) / 2.0;
                        line_parent_transform.translation.z =
                            point.z + (ball_transform.translation.z - point.z) / 2.0;

                        line_transform.scale.y = ball_transform
                            .translation
                            .distance(arrow_transform.translation)
                            - INNER_RADIUS_COEFF * 2.0;
                        line_parent_transform.look_at(point, Vec3::Z);

                        if !(mouse_button_input.just_released(MouseButton::Left)
                            || touches.any_just_released())
                        {
                            return;
                        }

                        let aim_direction = (point - ball_transform.translation).normalize();
                        vel.linvel = aim_direction * PROJECTILE_SPEED;

                        projectile_ball.is_flying = true;

                        pkv_play_shoot_audio(&mut commands, &audio_assets, &pkv);
                    } else {
                        *arrow_visibility = Visibility::Hidden;
                        *line_visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

pub fn setup_projectile_arrow(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
) {
    commands.spawn(ProjectileArrowBundle::new(
        Vec3::new(0.0, 0.0, PLAYER_SPAWN_Z / 2.0),
        &mut meshes,
        &gameplay_materials,
    ));
}

pub fn cleanup_projectile_arrow(
    mut commands: Commands,
    projectile_query: Query<Entity, With<ProjectileArrow>>,
) {
    for projectile_entity in projectile_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

pub fn setup_projectile_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Res<GameplayMaterials>,
) {
    commands
        .spawn(ProjectileLineBundle::new_parent(Vec3::new(
            0.0,
            0.0,
            PLAYER_SPAWN_Z / 2.0,
        )))
        .with_children(|parent| {
            parent.spawn(ProjectileLineBundle::new_child(
                Vec3::ZERO,
                &mut meshes,
                &gameplay_materials,
            ));
        });
}

pub fn cleanup_projectile_line(
    mut commands: Commands,
    projectile_query: Query<Entity, With<ProjectileLineParent>>,
) {
    for projectile_entity in projectile_query.iter() {
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
        if grid_ball_out.started && ball_transform.translation.z > PLAYER_SPAWN_Z {
            commands.entity(ball_entity).despawn_recursive();
        }
    }
}
