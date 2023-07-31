use bevy::{
    prelude::{Camera, GlobalTransform, Input, MouseButton, Query, Res, Touches, Vec2, With},
    window::{PrimaryWindow, Window},
};

use super::main_camera::components::MainCamera;

pub fn detect_pointer_position(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: &Res<Input<MouseButton>>,
    touches: &Res<Touches>,
) -> (Vec2, bool) {
    let mut pointer_position = Vec2::ZERO;
    let mut pointer_aquired = false;
    if let Ok(window) = window_query.get_single() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if mouse_button_input.pressed(MouseButton::Left)
                || mouse_button_input.just_released(MouseButton::Left)
            {
                if let Some(cursor_position) = window.cursor_position() {
                    if let Some(ray) =
                        camera.viewport_to_world_2d(camera_transform, cursor_position)
                    {
                        pointer_position = ray;
                        pointer_aquired = true;
                    }
                }
            }
            if let Some(touch) = touches.iter().next() {
                let touch_position = touch.position();
                if let Some(ray) = camera.viewport_to_world_2d(camera_transform, touch_position) {
                    pointer_position = ray;
                    pointer_aquired = true;
                }
            } else if let Some(touch) = touches.iter_just_released().next() {
                let touch_position = touch.position();
                if let Some(ray) = camera.viewport_to_world_2d(camera_transform, touch_position) {
                    pointer_position = ray;
                    pointer_aquired = true;
                }
            }
        }
    }
    (pointer_position, pointer_aquired)
}
