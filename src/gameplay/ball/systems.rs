use bevy::{
    prelude::{
        default, AudioBundle, Camera, Color, Commands, DespawnRecursiveExt, Entity, EventReader,
        EventWriter, GlobalTransform, Input, MouseButton, PlaybackSettings, Query, Res, ResMut,
        Transform, Vec3, With,
    },
    window::{PrimaryWindow, Window},
};
use bevy_prototype_debug_lines::DebugLines;
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
};

use super::{
    components::{GridBall, ProjectileBall, Species},
    constants::PROJECTILE_SPEED,
    events::SnapProjectile,
    projectile_ball_bundle::ProjectileBallBundle,
    resources::ProjectileBuffer,
};

pub fn rotate_projectile(
    mut query: Query<(&mut Transform, &ProjectileBall), With<ProjectileBall>>,
) {
    for (mut _transform, projectile_ball) in query.iter_mut() {
        if projectile_ball.is_flying {
            // transform.rotation *= Quat::from_rotation_z(0.1);
        }
    }
}

pub fn cleanup_projectile(
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

pub fn aim_projectile(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut projectile: Query<
        (Entity, &Transform, &mut Velocity, &mut ProjectileBall),
        With<ProjectileBall>,
    >,
    mouse: Res<Input<MouseButton>>,
    mut lines: ResMut<DebugLines>,
    audio_assets: Res<AudioAssets>,
) {
    if let Ok((_, transform, mut vel, mut projectile_ball)) = projectile.get_single_mut() {
        if projectile_ball.is_flying {
            return;
        }
        let (camera, camera_transform) = cameras.single();
        let (ray_pos, ray_dir) =
            ray_from_mouse_position(window_query.get_single().unwrap(), camera, camera_transform);
        let (plane_pos, plane_normal) = (Vec3::new(0., transform.translation.y, 0.), Vec3::Y);

        let mut point = plane_intersection(ray_pos, ray_dir, plane_pos, plane_normal);
        point.y = 0.0;

        // should use an angle instead
        point.z = point.z.min(transform.translation.z - 5.);

        lines.line_colored(transform.translation, point, 0.0, Color::GREEN);

        if !mouse.just_pressed(MouseButton::Left) {
            return;
        }

        commands.spawn((AudioBundle {
            source: audio_assets.flying.clone_weak(),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        },));

        let aim_direction = (point - transform.translation).normalize();
        vel.linvel = aim_direction * PROJECTILE_SPEED;

        projectile_ball.is_flying = true;
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
