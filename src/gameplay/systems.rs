use bevy::{
    prelude::{
        warn, Entity, EventReader, EventWriter, Input, KeyCode, NextState, Query, Res, ResMut,
        With, Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_pkv::PkvStore;
use bevy_xpbd_2d::prelude::Position;

use crate::components::AppState;

use super::{
    ball::components::{GridBall, OutBall, ProjectileBall},
    constants::GAME_OVER_BOTTOM,
    events::{FindCluster, MoveDownTopWall, ProjectileReload, SnapProjectile, SpawnRow},
    grid::resources::Grid,
    lines::components::LineType,
    utils::increment_init_rows,
};

pub fn setup_first_turn(mut begin_turn: EventWriter<ProjectileReload>) {
    begin_turn.send(ProjectileReload);
}

pub fn check_game_over(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines_query: Query<&mut Position, With<LineType>>,
    balls_query: Query<
        (Entity, &Position),
        (With<GridBall>, Without<ProjectileBall>, Without<LineType>),
    >,
) {
    let window = window_query.single();
    let game_over_bottom = -(window.height() - GAME_OVER_BOTTOM - window.height() / 2.0);

    for mut line_position in lines_query.iter_mut() {
        line_position.y = game_over_bottom
    }

    for (ball_entity, ball_position) in balls_query.iter() {
        if ball_position.y < game_over_bottom {
            warn!(
                "GameOver because ball {:?} position y ({}) < ({})",
                ball_entity, ball_position.y, game_over_bottom
            );
            app_state_next_state.set(AppState::GameOver);
            break;
        }
    }
}

pub fn check_game_win(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut grid: ResMut<Grid>,
    mut pkv: ResMut<PkvStore>,
    balls_query: Query<&GridBall, (With<GridBall>, Without<ProjectileBall>)>,
    out_balls_query: Query<&OutBall, With<OutBall>>,
) {
    if balls_query.iter().len() == 0 && out_balls_query.iter().count() == 0 {
        increment_init_rows(grid.as_mut(), &mut pkv);
        app_state_next_state.set(AppState::GameWin);
    }
}

pub fn keydown_detect(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
    mut grid: ResMut<Grid>,
    mut pkv: ResMut<PkvStore>,
) {
    if keyboard_input_key_code.any_just_released([KeyCode::Escape]) {
        app_state_next_state.set(AppState::GameOver);
    }
    if keyboard_input_key_code.any_just_released([KeyCode::Space]) {
        increment_init_rows(grid.as_mut(), &mut pkv);
        app_state_next_state.set(AppState::GameWin);
    }
}

pub fn cleanup_events(
    mut projectile_reload_events: EventReader<ProjectileReload>,
    mut snap_projectile_events: EventReader<SnapProjectile>,
    mut move_down_events: EventReader<MoveDownTopWall>,
    mut spawn_row_events: EventReader<SpawnRow>,
    mut find_cluster_events: EventReader<FindCluster>,
) {
    if projectile_reload_events.len() > 0 {
        projectile_reload_events.clear();
    }
    if snap_projectile_events.len() > 0 {
        snap_projectile_events.clear();
    }
    if move_down_events.len() > 0 {
        move_down_events.clear();
    }
    if spawn_row_events.len() > 0 {
        spawn_row_events.clear();
    }
    if find_cluster_events.len() > 0 {
        find_cluster_events.clear();
    }
}
