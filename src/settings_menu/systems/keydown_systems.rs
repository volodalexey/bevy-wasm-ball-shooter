use bevy::prelude::{Input, KeyCode, NextState, Res, ResMut};

use crate::components::AppState;

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Escape]) {
        app_state_next_state.set(AppState::StartMenu);
    }
}
