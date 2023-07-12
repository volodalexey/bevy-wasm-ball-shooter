use bevy::prelude::{
    default, Camera3dBundle, Commands, DespawnRecursiveExt, Entity, Input, KeyCode,
    PerspectiveProjection, Projection, Query, Res, Transform, Vec3, With,
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

pub fn control_camera_position(
    keyboard_input_key_code: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera_transform = camera_query.single_mut();
    if keyboard_input_key_code.any_pressed([KeyCode::A, KeyCode::Left]) {
        camera_transform.translation += Vec3::new(-0.5, 0.0, 0.0)
    }
    if keyboard_input_key_code.any_pressed([KeyCode::D, KeyCode::Right]) {
        camera_transform.translation += Vec3::new(0.5, 0.0, 0.0);
    }
    if keyboard_input_key_code.any_pressed([KeyCode::W, KeyCode::Up]) {
        camera_transform.translation += Vec3::new(0.0, 0.0, -0.5);
    }
    if keyboard_input_key_code.any_pressed([KeyCode::S, KeyCode::Down]) {
        camera_transform.translation += Vec3::new(0.0, 0.0, 0.5);
    }
}
