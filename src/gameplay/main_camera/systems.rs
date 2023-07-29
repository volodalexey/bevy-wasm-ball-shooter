use bevy::prelude::{
    Camera2dBundle, Commands, DespawnRecursiveExt, Entity, Input, KeyCode, Query, Res, Transform,
    Vec3, With,
};

use super::components::MainCamera;

pub fn setup_main_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
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
        camera_transform.translation += Vec3::new(0.0, -0.5, 0.0);
    }
    if keyboard_input_key_code.any_pressed([KeyCode::S, KeyCode::Down]) {
        camera_transform.translation += Vec3::new(0.0, 0.5, 0.0);
    }
}
