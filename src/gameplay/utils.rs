use bevy::{
    prelude::{Camera, GlobalTransform, Input, MouseButton, Query, Res, Touches, Vec2, With},
    window::{PrimaryWindow, Window},
};

use crate::resources::LevelCounter;

use super::{constants::MAX_LEVEL, main_camera::components::MainCamera};

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

pub fn calc_init_cols_rows(level_counter: &Res<LevelCounter>) -> (i32, i32) {
    (
        match level_counter.0 {
            1 => 6,
            2 => 6,
            3 => 6,
            4 => 7,
            5 => 7,
            6 => 8,
            7 => 8,
            8 => 9,
            9 => 9,
            10 => 9,
            11 => 10,
            12 => 10,
            13 => 11,
            14 => 11,
            15 => 12,
            MAX_LEVEL => 12,
            _ => 0,
        },
        match level_counter.0 {
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 6,
            5 => 8,
            6 => 10,
            7 => 12,
            8 => 14,
            9 => 16,
            10 => 18,
            11 => 20,
            12 => 22,
            13 => 24,
            14 => 26,
            15 => 28,
            MAX_LEVEL => 30,
            _ => 0,
        },
    )
}
