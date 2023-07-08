use bevy::prelude::{
    default, Camera3dBundle, Commands, DespawnRecursiveExt, Entity, PerspectiveProjection,
    Projection, Query, Transform, Vec3, With,
};

use crate::gameplay::constants::PLAYER_SPAWN_Z;

use super::components::MainCamera;

pub fn setup_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 76.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 70.0, 41.0)
                .looking_at(Vec3::new(0.0, 0.0, PLAYER_SPAWN_Z / 2.), Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}

pub fn cleanup_main_camera(mut commands: Commands, query: Query<Entity, With<MainCamera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
