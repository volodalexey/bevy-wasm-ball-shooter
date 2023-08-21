use bevy::{
    prelude::{
        warn, Entity, EventWriter, Input, KeyCode, NextState, Query, Res, ResMut, Transform, With,
        Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use hexx::Hex;

use crate::{components::AppState, resources::LevelCounter, utils::increment_level};

use super::{
    ball::components::{GridBall, OutBall, ProjectileBall},
    constants::GAME_OVER_BOTTOM,
    events::ProjectileReload,
    grid::resources::Grid,
    lines::components::LineType,
};

pub fn setup_first_turn(mut begin_turn: EventWriter<ProjectileReload>) {
    begin_turn.send(ProjectileReload);
}

pub fn check_game_over(
    grid: Res<Grid>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines_query: Query<(&LineType, &mut Transform), With<LineType>>,
    balls_query: Query<
        (Entity, &Transform),
        (With<GridBall>, Without<ProjectileBall>, Without<LineType>),
    >,
) {
    let window = window_query.single();
    let game_over_bottom = -(window.height() - GAME_OVER_BOTTOM - window.height() / 2.0);

    let hex = Hex {
        x: 0,
        y: grid.last_active_row,
    };
    let position = grid.layout.hex_to_world_pos(hex);

    for (line_type, mut line_transform) in lines_query.iter_mut() {
        match line_type {
            LineType::GridTop => line_transform.translation.y = position.y,
            LineType::GameOver => line_transform.translation.y = game_over_bottom,
        }
    }

    for (ball_entity, ball_transform) in balls_query.iter() {
        if ball_transform.translation.y < game_over_bottom {
            warn!(
                "GameOver because ball {:?} position y ({}) < ({})",
                ball_entity, ball_transform.translation.y, game_over_bottom
            );
            app_state_next_state.set(AppState::GameOver);
            break;
        }
    }
}

pub fn check_game_win(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut level_counter: ResMut<LevelCounter>,
    mut pkv: ResMut<PkvStore>,
    balls_query: Query<&GridBall, (With<GridBall>, Without<ProjectileBall>)>,
    out_balls_query: Query<&OutBall, With<OutBall>>,
) {
    if balls_query.iter().len() == 0 && out_balls_query.iter().count() == 0 {
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
