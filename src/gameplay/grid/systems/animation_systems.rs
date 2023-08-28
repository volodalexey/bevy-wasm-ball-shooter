use bevy::{
    prelude::{
        Commands, Entity, EventReader, EventWriter, Input, KeyCode, Query, Res, ResMut, Vec2, With,
        Without,
    },
    time::Time,
    utils::HashSet,
    window::{PrimaryWindow, Window},
};
use bevy_xpbd_2d::prelude::{AngularVelocity, LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, GridBallPositionAnimate, ProjectileBall},
    constants::{LOG_KEYCODE_MOVE_DOWN, MOVE_DOWN_TOLERANCE, ROW_HEIGHT},
    events::{FindCluster, MoveDownLastActive, SpawnRow},
    grid::{
        resources::{CooldownMoveCounter, Grid},
        utils::adjust_grid_layout,
    },
    panels::resources::MoveDownCounter,
};

pub fn move_down_grid_balls(
    mut commands: Commands,
    balls_query: Query<
        (
            Entity,
            &Position,
            &RigidBody,
            &GridBall,
            Option<&GridBallPositionAnimate>,
        ),
        (With<GridBall>, Without<ProjectileBall>),
    >,
    mut move_down_events: EventReader<MoveDownLastActive>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut grid: ResMut<Grid>,
    mut move_counter: ResMut<MoveDownCounter>,
    mut cooldown_move_counter: ResMut<CooldownMoveCounter>,
    time: Res<Time>,
    mut writer_spawn_row: EventWriter<SpawnRow>,
) {
    if !cooldown_move_counter.timer.paused() {
        cooldown_move_counter.timer.tick(time.delta());
    }
    if cooldown_move_counter.timer.finished() {
        cooldown_move_counter.timer.pause();
        cooldown_move_counter.timer.reset();
        writer_spawn_row.send(SpawnRow);
    }
    if move_down_events.is_empty() {
        return;
    }
    move_down_events.clear();

    cooldown_move_counter.value -= 1;
    if cooldown_move_counter.value == 0 {
        move_counter.0 += 1;
        cooldown_move_counter.reset();

        adjust_grid_layout(&window_query, &mut grid, &move_counter);
        for (ball_entity, ball_position, rigid_body, grid_ball, some_ball_animate) in
            balls_query.iter()
        {
            if !grid_ball.is_ready_to_despawn && rigid_body.is_kinematic() {
                let position = match some_ball_animate {
                    Some(ball_animate) => ball_animate.position,
                    None => ball_position.0,
                } - Vec2::new(0.0, ROW_HEIGHT);
                commands
                    .entity(ball_entity)
                    .insert(GridBallPositionAnimate::from_position(position));
            }
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
    mut writer_find_cluster: EventWriter<FindCluster>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    let mut to_check: HashSet<Entity> = HashSet::default();
    for (
        ball_entity,
        ball_position,
        mut grid_ball_animate,
        mut linear_velocity,
        mut angular_velocity,
    ) in grid_balls_query.iter_mut()
    {
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
        if keyboard_input_key_code.any_just_released([LOG_KEYCODE_MOVE_DOWN]) {
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
            to_check.insert(ball_entity);
        }
    }
    if to_check.len() > 0 {
        writer_find_cluster.send(FindCluster { to_check });
    }
}
