use bevy::{
    prelude::{
        Assets, Camera, Color, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter,
        GlobalTransform, Input, Mesh, MouseButton, Quat, Query, Res, ResMut, StandardMaterial,
        Transform, Vec3, With,
    },
    window::{PrimaryWindow, Window},
};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::{Collider, CollisionEvent, Velocity};

use crate::{
    gameplay::{
        ball::{random_species, Ball},
        constants::PLAYER_SPAWN_Z,
        events::BeginTurn,
        grid::resources::Grid,
        main_camera::components::MainCamera,
        projectile::utils::clamp_inside_world_bounds,
        utils::{plane_intersection, ray_from_mouse_position},
    },
    loading::audio_assets::{events::AudioEvent, AudioAssets},
};

use super::{
    bundles::ProjectileBundle,
    components::{Flying, Projectile},
    constants::PROJECTILE_SPEED,
    events::SnapProjectile,
    resources::ProjectileBuffer,
};

pub fn rotate_projectile(
    mut query: Query<(&mut Transform, &Flying), (With<Projectile>, With<Flying>)>,
) {
    for (mut transform, flying) in query.iter_mut() {
        if flying.0 {
            transform.rotation *= Quat::from_rotation_z(0.1);
        }
    }
}

pub fn cleanup_projectile(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
) {
    for projectile_entity in projectile_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}

pub fn projectile_reload(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        None => random_species(),
    };

    commands.spawn(ProjectileBundle::new(
        Vec3::new(0.0, 0.0, PLAYER_SPAWN_Z),
        grid.layout.size.x,
        species,
        &mut meshes,
        &mut materials,
    ));

    buffer.0.push(random_species());
}

pub fn aim_projectile(
    window_query: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut projectile: Query<(Entity, &Transform, &mut Velocity, &mut Flying), With<Flying>>,
    mouse: Res<Input<MouseButton>>,
    mut lines: ResMut<DebugLines>,
    audio_assets: Res<AudioAssets>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    if let Ok((_, transform, mut vel, mut is_flying)) = projectile.get_single_mut() {
        if is_flying.0 {
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

        audio_event.send(AudioEvent {
            clip: audio_assets.flying.clone_weak(),
        });

        let aim_direction = (point - transform.translation).normalize();
        vel.linvel = aim_direction * PROJECTILE_SPEED;

        is_flying.0 = true;
    }
}

pub fn bounce_on_world_bounds(
    mut projectile: Query<
        (Entity, &mut Transform, &mut Velocity, &Collider, &Flying),
        With<Flying>,
    >,
    mut snap_projectile: EventWriter<SnapProjectile>,
    grid: Res<Grid>,
) {
    if let Ok((_, mut transform, mut vel, collider, flying)) = projectile.get_single_mut() {
        if !flying.0 {
            return;
        }
        if let Some(shape) = collider.raw.as_ball() {
            const SKIN_WIDTH: f32 = 0.1;
            let skin = shape.radius + SKIN_WIDTH;

            let (clamped, was_clamped_x, was_clamped_y) =
                clamp_inside_world_bounds(transform.translation, skin, &grid.bounds);

            transform.translation = clamped;

            if was_clamped_x {
                vel.linvel.x = -vel.linvel.x;
            }

            // We hit the top, snap ball
            if was_clamped_y {
                vel.linvel = Vec3::ZERO;
                snap_projectile.send(SnapProjectile {
                    entity: None,
                    hit_normal: None,
                });
            }
        }
    }
}

pub fn on_projectile_collisions_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut snap_projectile: EventWriter<SnapProjectile>,
    mut projectile: Query<
        (Entity, &mut Velocity, &Transform, &Flying),
        (With<Projectile>, With<Flying>),
    >,
    balls: Query<(Entity, &Transform), With<Ball>>,
) {
    for (d1, d2, _) in collision_events.iter().filter_map(|e| match e {
        CollisionEvent::Started(a, b, f) => Some((a, b, f)),
        CollisionEvent::Stopped(_, _, _) => None,
    }) {
        let mut p1 = projectile.get_mut(*d1);
        if p1.is_err() {
            p1 = projectile.get_mut(*d2);
        }

        if let Ok((entity, otr)) = balls.get(*d1).or(balls.get(*d2)) {
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
