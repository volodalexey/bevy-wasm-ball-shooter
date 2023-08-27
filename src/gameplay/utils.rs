use bevy::{
    prelude::{
        Camera, GlobalTransform, Input, MouseButton, Query, Res, ResMut, Touches, Vec2, With,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;

use crate::constants::{MAX_ROWS_COUNT, MIN_ROWS_COUNT, TOTAL_ROWS_KEY};

use super::{grid::resources::Grid, main_camera::components::MainCamera};

pub fn detect_pointer_position(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: &Res<Input<MouseButton>>,
    touches: &Res<Touches>,
) -> (Vec2, bool, bool, bool) {
    let mut some_global_pointer_position: Option<Vec2> = None;
    let mut pointer_position = Vec2::ZERO;
    let mut pointer_aquired = false;
    let is_mouse_down = mouse_button_input.pressed(MouseButton::Left)
        || mouse_button_input.just_pressed(MouseButton::Left);
    let is_mouse_up = mouse_button_input.just_released(MouseButton::Left);
    let mut is_pressed = false;
    let mut is_released = is_mouse_up;
    if is_mouse_down || is_mouse_up {
        if let Ok(window) = window_query.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                if is_mouse_down {
                    is_pressed = true;
                }
                some_global_pointer_position = Some(cursor_position);
            }
        }
    } else if let Some(touch) = touches.iter().next() {
        let touch_position = touch.position();
        some_global_pointer_position = Some(touch_position);
        is_pressed = true;
    } else if let Some(touch) = touches.iter_just_released().next() {
        let touch_position = touch.position();
        some_global_pointer_position = Some(touch_position);
        is_released = true;
    }
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        if let Some(global_pointer_position) = some_global_pointer_position {
            if let Some(ray) =
                camera.viewport_to_world_2d(camera_transform, global_pointer_position)
            {
                pointer_position = ray;
                pointer_aquired = true;
            }
        }
    }
    (pointer_position, is_pressed, is_released, pointer_aquired)
}

pub fn increment_init_rows(grid: &mut Grid, pkv: &mut ResMut<PkvStore>) {
    grid.total_rows += 1;
    if grid.total_rows > MAX_ROWS_COUNT {
        grid.total_rows = MIN_ROWS_COUNT
    }
    pkv.set_string(TOTAL_ROWS_KEY, &grid.total_rows.to_string())
        .expect("failed to save total rows");
}
