use bevy::{
    prelude::{
        default, Assets, AudioBundle, Camera, Commands, DespawnRecursiveExt, Entity, EventReader,
        EventWriter, GlobalTransform, Input, Mesh, MouseButton, PlaybackSettings, Query, Res,
        ResMut, Touches, Transform, Vec2, Vec3, Visibility, With, Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};

use crate::{
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
    resources::PointerCooldown,
};

use super::{
    components::{GridBall, ProjectileArrow, ProjectileBall, Species},
    constants::PROJECTILE_SPEED,
    events::SnapProjectile,
    projectile_arrow_bundle::ProjectileArrowBundle,
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
) {
    if begin_turn.is_empty() {
        return;
    }
    begin_turn.clear();

    let species = match buffer.0.pop() {
        Some(species) => species,
        None => Species::random_species(),
    };

    commands.spawn(ProjectileBallBundle::new(
        Vec3::new(0.0, 0.0, PLAYER_SPAWN_Z),
        grid.layout.hex_size.x,
        species,
        &gameplay_meshes,
        &gameplay_materials,
    ));

    buffer.0.push(Species::random_species());
}

pub fn shoot_projectile(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut projectile_ball_query: Query<
        (&Transform, &mut Velocity, &mut ProjectileBall),
        (With<ProjectileBall>, Without<ProjectileArrow>),
    >,
    mouse_button_input: Res<Input<MouseButton>>,
    audio_assets: Res<AudioAssets>,
    mut projectile_arrow_query: Query<
        (&mut Transform, &mut Visibility),
        (With<ProjectileArrow>, Without<ProjectileBall>),
    >,
    touches: Res<Touches>,
    pointer_cooldown: Res<PointerCooldown>,
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
    if let Ok((mut arrow_transform, mut visibility)) = projectile_arrow_query.get_single_mut() {
        if let Ok((ball_transform, mut vel, mut projectile_ball)) =
            projectile_ball_query.get_single_mut()
        {
            if pointer_position.length() > Vec2::ZERO.length() && !projectile_ball.is_flying {
                *visibility = Visibility::Visible;

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

                let mut point = plane_intersection(ray_pos, ray_dir, plane_pos, plane_normal);
                point.y = 0.0;

                // lines.line_colored(transform.translation, point, 0.0, Color::GREEN);
                arrow_transform.translation = point;

                if !(mouse_button_input.just_released(MouseButton::Left)
                    || touches.any_just_released())
                {
                    return;
                }

                commands.spawn((AudioBundle {
                    source: audio_assets.flying.clone_weak(),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                },));

                let aim_direction = (point - ball_transform.translation).normalize();
                vel.linvel = aim_direction * PROJECTILE_SPEED;

                projectile_ball.is_flying = true;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn on_projectile_collisions_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut snap_projectile: EventWriter<SnapProjectile>,
    mut projectile_query: Query<
        (Entity, &mut Velocity, &Transform, &ProjectileBall),
        With<ProjectileBall>,
    >,
    balls_query: Query<(Entity, &Transform), With<GridBall>>,
) {
    for (d1, d2, _) in collision_events.iter().filter_map(|e| match e {
        CollisionEvent::Started(a, b, f) => Some((a, b, f)),
        CollisionEvent::Stopped(_, _, _) => None,
    }) {
        let mut p1 = projectile_query.get_mut(*d1);
        if p1.is_err() {
            p1 = projectile_query.get_mut(*d2);
        }

        if let Ok((entity, otr)) = balls_query.get(*d1).or(balls_query.get(*d2)) {
            let (_, mut vel, tr, _) = p1.unwrap();
            let hit_normal = (otr.translation - tr.translation).normalize();
            vel.linvel = Vec3::ZERO;
            snap_projectile.send(SnapProjectile {
                entity: Some(entity),
                hit_normal: Some(hit_normal),
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
