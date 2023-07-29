use bevy::prelude::{
    warn, EventReader, EventWriter, Input, KeyCode, NextState, Query, Res, ResMut, With,
};
use bevy_pkv::PkvStore;
use hexx::Direction;

use crate::{components::AppState, resources::LevelCounter, utils::increment_level};

use super::{
    ball::components::{GridBall, OutBall},
    constants::PROJECTILE_SPAWN,
    events::BeginTurn,
    grid::{events::UpdatePositions, resources::Grid},
};

pub fn setup_gameplay(mut begin_turn: EventWriter<BeginTurn>) {
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

pub fn check_game_over(grid: Res<Grid>, mut app_state_next_state: ResMut<NextState<AppState>>) {
    let projectile_hex = grid.layout.world_pos_to_hex(hexx::Vec2 {
        x: 0.0,
        y: PROJECTILE_SPAWN,
    });
    let hex_game_over = projectile_hex
        .neighbor(Direction::Top)
        .neighbor(Direction::Top);

    let hex_game_over_pos = grid.layout.hex_to_world_pos(hex_game_over);

    for (&hex_grid, _) in grid.storage.iter() {
        let hex_grid_pos = grid.layout.hex_to_world_pos(hex_grid);
        if hex_grid_pos.y >= hex_game_over_pos.y {
            warn!(
                "GameOver because hex({}, {}) position is ({} < {})",
                hex_grid.x, hex_grid.y, hex_grid_pos.y, hex_game_over_pos.y
            );
            app_state_next_state.set(AppState::GameOver);
            break;
        }
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
