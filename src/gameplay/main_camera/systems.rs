use bevy::{
    input::mouse::MouseWheel,
    prelude::{
        default, Camera2dBundle, Commands, DespawnRecursiveExt, Entity, EventReader, Input,
        KeyCode, Quat, Query, Res, Transform, Vec3, With,
    },
};

use crate::gameplay::constants::{CAMERA_ROTATION_SPEED, CAMERA_SCALE, CAMERA_SPEED};

use super::components::MainCamera;

pub fn setup_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            // transform: Transform::from_rotation(Quat::from_rotation_z(core::f32::consts::PI)),
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
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let mut direction = Vec3::ZERO;
    let mut camera_transform = camera_query.single_mut();
    if keyboard_input_key_code.any_pressed([KeyCode::A]) {
        direction += Vec3::new(-1.0, 0.0, 0.0)
    }
    if keyboard_input_key_code.any_pressed([KeyCode::D]) {
        direction += Vec3::new(1.0, 0.0, 0.0)
    }
    if keyboard_input_key_code.any_pressed([KeyCode::W]) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input_key_code.any_pressed([KeyCode::S]) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction.length() > 0.0 {
        direction = direction.normalize_or_zero();
        camera_transform.translation += direction * CAMERA_SPEED;
    }

    if keyboard_input_key_code.any_pressed([KeyCode::Q]) {
        camera_transform.rotation *= Quat::from_rotation_z(CAMERA_ROTATION_SPEED);
    }
    if keyboard_input_key_code.any_pressed([KeyCode::E]) {
        camera_transform.rotation *= Quat::from_rotation_z(-CAMERA_ROTATION_SPEED);
    }
    if keyboard_input_key_code.any_pressed([KeyCode::R]) {
        camera_transform.rotation = Quat::IDENTITY;
        camera_transform.translation = Vec3::ZERO;
        camera_transform.scale = Vec3::ONE;
    }
    for ev in mouse_wheel_events.iter() {
        match ev.y.signum() > 0.0 {
            true => {
                camera_transform.scale.x += CAMERA_SCALE;
                camera_transform.scale.y += CAMERA_SCALE;
            }
            false => {
                camera_transform.scale.x -= CAMERA_SCALE;
                camera_transform.scale.y -= CAMERA_SCALE;
            }
        }
    }
}
