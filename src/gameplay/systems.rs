use bevy::{
    prelude::{
        warn, EventReader, EventWriter, Input, KeyCode, NextState, Query, Res, ResMut, Transform,
        With,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;

use crate::{components::AppState, resources::LevelCounter, utils::increment_level};

use super::{
    ball::components::{GridBall, OutBall},
    constants::GAME_OVER_BOTTOM,
    events::BeginTurn,
    grid::{events::UpdatePositions, resources::Grid},
    lines::components::LineType,
};

pub fn setup_first_turn(mut begin_turn: EventWriter<BeginTurn>) {
    begin_turn.send(BeginTurn);
}

pub fn on_begin_turn(
    mut begin_turn: EventReader<BeginTurn>,
    mut update_positions: EventWriter<UpdatePositions>,
) {
    if begin_turn.is_empty() {
        return;
    }
    begin_turn.clear();

    update_positions.send(UpdatePositions);
}

pub fn check_game_over(
    mut grid: ResMut<Grid>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines_query: Query<(&LineType, &mut Transform), With<LineType>>,
) {
    let window = window_query.single();
    let game_over_bottom = -(window.height() - GAME_OVER_BOTTOM - window.height() / 2.0);

    for (line_type, mut line_transform) in lines_query.iter_mut() {
        match line_type {
            LineType::GridTop | LineType::GridBottom => {}
            LineType::GameOver => line_transform.translation.y = game_over_bottom,
        }
    }

    grid.check_update_bounds();
    if grid.bounds.mins.y < game_over_bottom {
        warn!(
            "GameOver because minimal bound ({}) less than ({})",
            grid.bounds.mins.y, game_over_bottom
        );
        app_state_next_state.set(AppState::GameOver);
    }
}

pub fn check_game_win(
    grid: Res<Grid>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut level_counter: ResMut<LevelCounter>,
    mut pkv: ResMut<PkvStore>,
    balls_query: Query<&GridBall, With<GridBall>>,
    out_balls_query: Query<&OutBall, With<OutBall>>,
) {
    if grid.storage.len() == 0
        && balls_query.iter().len() == 0
        && out_balls_query.iter().count() == 0
    {
        increment_level(&mut level_counter, &mut pkv);
        app_state_next_state.set(AppState::GameWin);
    }
}

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
    mut level_counter: ResMut<LevelCounter>,
    mut pkv: ResMut<PkvStore>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Escape]) {
        app_state_next_state.set(AppState::GameOver);
    }
    if keyboard_input_key_code.any_just_released([KeyCode::Space]) {
        increment_level(&mut level_counter, &mut pkv);
        app_state_next_state.set(AppState::GameWin);
    }
}
