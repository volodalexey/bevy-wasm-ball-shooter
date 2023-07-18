use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::{
        default, Camera3dBundle, Commands, DespawnRecursiveExt, Entity, EventReader, Input,
        KeyCode, MouseButton, PerspectiveProjection, Projection, Quat, Query, Res, Transform, Vec3,
        With,
    },
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
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut motion_motion_events: EventReader<MouseMotion>,
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
    for ev in mouse_wheel_events.iter() {
        camera_transform.translation += Vec3::new(0.0, -ev.y, 0.0);
    }
    if mouse_button_input.pressed(MouseButton::Middle) {
        for ev in motion_motion_events.iter() {
            let rotation = ev.delta.x / 10.0;
            // camera_transform.rotate_x(rotation);
            if rotation >= 0.0 {
                camera_transform.rotation *= Quat::from_rotation_x(rotation);
            } else {
                camera_transform.rotation *= Quat::from_rotation_x(rotation);
            }
        }
    }
}
