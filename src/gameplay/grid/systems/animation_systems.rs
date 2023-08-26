use bevy::{
    prelude::{
        Commands, Entity, EventReader, EventWriter, Input, KeyCode, Query, Res, ResMut, Vec2, With,
        Without,
    },
    time::Time,
    window::{PrimaryWindow, Window},
};
use bevy_xpbd_2d::prelude::{AngularVelocity, LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, GridBallPositionAnimate, ProjectileBall},
    constants::{FILL_PLAYGROUND_ROWS, MOVE_DOWN_TOLERANCE, ROW_HEIGHT},
    events::{MoveDownLastActive, SpawnRow},
    grid::{resources::Grid, utils::adjust_grid_layout},
    panels::resources::MoveCounter,
};

pub fn move_down_grid_balls(
    mut commands: Commands,
    balls_query: Query<
        (
            Entity,
            &Position,
            &RigidBody,
            Option<&GridBallPositionAnimate>,
        ),
        (With<GridBall>, Without<ProjectileBall>),
    >,
    mut move_down_events: EventReader<MoveDownLastActive>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut grid: ResMut<Grid>,
    move_counter: Res<MoveCounter>,
) {
    if move_down_events.is_empty() {
        return;
    }
    move_down_events.clear();

    adjust_grid_layout(&window_query, &mut grid, &move_counter);
    for (ball_entity, ball_position, rigid_body, some_ball_animate) in balls_query.iter() {
        if rigid_body.is_kinematic() {
            let position = match some_ball_animate {
                Some(ball_animate) => ball_animate.position,
                None => ball_position.0,
            } - Vec2::new(0.0, ROW_HEIGHT);
            commands
                .entity(ball_entity)
                .insert(GridBallPositionAnimate::from_position(position, true));
        }
    }
}

pub fn animate_grid_ball_position(
    mut commands: Commands,
    mut grid_balls_query: Query<
        (
            Entity,
            &Position,
            &mut GridBallPositionAnimate,
            &mut LinearVelocity,
            &mut AngularVelocity,
        ),
        With<GridBallPositionAnimate>,
    >,
    time: Res<Time>,
    grid: Res<Grid>,
    move_counter: Res<MoveCounter>,
    mut writer_spawn_row: EventWriter<SpawnRow>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    let mut total_count: u32 = 0;
    let mut completed_count: u32 = 0;
    for (
        ball_entity,
        ball_position,
        mut grid_ball_animate,
        mut linear_velocity,
        mut angular_velocity,
    ) in grid_balls_query.iter_mut()
    {
        if grid_ball_animate.move_down_after {
            total_count += 1;
        }
        grid_ball_animate.timer.tick(time.delta());
        // linear_velocity.0 = ball_position.lerp(
        //     grid_ball_animate.position,
        //     grid_ball_animate.timer.percent(),
        // );
        linear_velocity.0 = grid_ball_animate.position - ball_position.0;
        // linear_velocity.0 += diff_normilized * 2.0;
        let position_diff_length = (ball_position.0 - grid_ball_animate.position).length();
        // println!(
        //     "animate {:?} ball_position {} animate_position {} position_diff_length {} linear_velocity {}",
        //     ball_entity, ball_position.0, grid_ball_animate.position, position_diff_length, linear_velocity.0
        // );
        if keyboard_input_key_code.any_just_released([KeyCode::K]) {
            println!(
                "move down entity {:?} linear_velocity {} position_diff_length {}",
                ball_entity, linear_velocity.0, position_diff_length
            );
        }
        if position_diff_length < MOVE_DOWN_TOLERANCE {
            linear_velocity.0 = Vec2::ZERO;
            angular_velocity.0 = 0.0;
            commands
                .entity(ball_entity)
                .remove::<GridBallPositionAnimate>();
            if grid_ball_animate.move_down_after {
                completed_count += 1;
            }
        }
    }
    if completed_count == total_count && completed_count > 0 {
        if grid.init_rows - FILL_PLAYGROUND_ROWS > move_counter.0 as i32 - 1 {
            writer_spawn_row.send(SpawnRow);
        }
        // TODO find cluster for all static balls
    }
}
