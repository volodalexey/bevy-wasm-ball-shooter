use bevy::{
    prelude::{
        Assets, BuildChildren, Camera, Commands, DespawnRecursiveExt, Entity, EventReader,
        EventWriter, GlobalTransform, Input, Mesh, MouseButton, Query, Res, ResMut, Touches,
        Transform, Vec2, Vec3, Visibility, With, Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};

use crate::{
    game_audio::{constants::SHOOT_SOUND_VOLUME_KEY, utils::play_shoot_audio},
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
    resources::{LevelCounter, PointerCooldown},
};

use super::{
    components::{
        GridBall, ProjectileArrow, ProjectileBall, ProjectileLine, ProjectileLineParent, Species,
    },
    constants::{INNER_RADIUS_COEFF, PROJECTILE_SPEED},
    events::SnapProjectile,
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
    level_counter: Res<LevelCounter>,
) {
    if begin_turn.is_empty() {
        return;
    }
    begin_turn.clear();

    let species = match buffer.0.pop() {
        Some(species) => species,
        None => Species::random_species(&level_counter),
    };

    commands.spawn(ProjectileBallBundle::new(
        Vec3::new(0.0, 0.0, PLAYER_SPAWN_Z),
        grid.layout.hex_size.x,
        species,
        &gameplay_meshes,
        &gameplay_materials,
    ));

    buffer.0.push(Species::random_species(&level_counter));
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

                        if let Ok(shoot_sound_volume) = pkv.get::<String>(SHOOT_SOUND_VOLUME_KEY) {
                            if let Ok(shoot_sound_volume) = shoot_sound_volume.parse::<f32>() {
                                if shoot_sound_volume > 0.0 {
                                    play_shoot_audio(
                                        &mut commands,
                                        &audio_assets,
                                        shoot_sound_volume,
                                    );
                                }
                            }
                        }
                    } else {
                        *arrow_visibility = Visibility::Hidden;
                        *line_visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

pub fn on_projectile_collisions_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut snap_projectile: EventWriter<SnapProjectile>,
    mut projectile_query: Query<(Entity, &Transform, &Species), With<ProjectileBall>>,
    balls_query: Query<(Entity, &Transform), With<GridBall>>,
) {
    for (entity_a, entity_b) in collision_events.iter().filter_map(|e| match e {
        CollisionEvent::Started(a, b, _) => Some((a, b)),
        CollisionEvent::Stopped(_, _, _) => None,
    }) {
        if let Ok((_, _)) = balls_query.get(*entity_a).or(balls_query.get(*entity_b)) {
            let mut p1 = projectile_query.get_mut(*entity_a);
            if p1.is_err() {
                p1 = projectile_query.get_mut(*entity_b);
            }

            let (projectile_entity, projectile_transform, species) = p1.unwrap();
            commands.entity(projectile_entity).despawn_recursive();
            snap_projectile.send(SnapProjectile {
                out_of_bounds: false,
                pos: Vec2::new(
                    projectile_transform.translation.x,
                    projectile_transform.translation.z,
                ),
                species: *species,
            });
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
