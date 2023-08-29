use bevy::{
    prelude::{Commands, Entity, EventReader, EventWriter, Query, ResMut, Vec2, With, Without},
    window::{PrimaryWindow, Window},
};
use bevy_xpbd_2d::prelude::{LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::components::GridBall,
    constants::{MOVE_DOWN_TOLERANCE, ROW_HEIGHT},
    events::{MoveDownTopWall, SpawnRow},
    grid::{
        resources::{CooldownMoveCounter, Grid},
        utils::adjust_grid_layout,
    },
    panels::resources::MoveDownCounter,
    walls::components::{TopWall, TopWallPositionAnimate},
};

pub fn move_down_top_wall(
    mut commands: Commands,
    mut top_wall_query: Query<
        (
            Entity,
            &mut Position,
            Option<&TopWallPositionAnimate>,
            &mut LinearVelocity,
        ),
        With<TopWall>,
    >,
    mut balls_query: Query<
        (&RigidBody, &mut Position, &mut LinearVelocity),
        (With<GridBall>, Without<TopWall>),
    >,
    mut move_down_events: EventReader<MoveDownTopWall>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut grid: ResMut<Grid>,
    mut move_counter: ResMut<MoveDownCounter>,
    mut cooldown_move_counter: ResMut<CooldownMoveCounter>,
    mut writer_spawn_row: EventWriter<SpawnRow>,
) {
    if let Some(_) = move_down_events.iter().next() {
        cooldown_move_counter.value -= 1;
        if cooldown_move_counter.value == 0 {
            move_counter.0 += 1;
            cooldown_move_counter.reset();

            adjust_grid_layout(&window_query, &mut grid, move_counter.0);
            for (wall_entity, wall_position, some_wall_animate, _) in top_wall_query.iter() {
                let position = match some_wall_animate {
                    Some(wall_animate) => wall_animate.position,
                    None => wall_position.0,
                } - Vec2::new(0.0, ROW_HEIGHT);
                commands
                    .entity(wall_entity)
                    .insert(TopWallPositionAnimate { position });
            }
        }
    }
    for (wall_entity, mut wall_position, some_wall_animate, mut wall_linear_velocity) in
        top_wall_query.iter_mut()
    {
        if let Some(wall_animate) = some_wall_animate {
            wall_linear_velocity.0 = wall_animate.position - wall_position.0;
            for (rigid_body, _, mut ball_linear_velocity) in balls_query.iter_mut() {
                if rigid_body.is_kinematic() {
                    ball_linear_velocity.0 = wall_linear_velocity.0
                }
            }
            let position_diff_length = (wall_position.0 - wall_animate.position).length();
            if position_diff_length < MOVE_DOWN_TOLERANCE {
                wall_linear_velocity.0 = Vec2::ZERO;
                commands
                    .entity(wall_entity)
                    .remove::<TopWallPositionAnimate>();
                wall_position.0 = wall_animate.position;

                writer_spawn_row.send(SpawnRow);
                for (rigid_body, mut ball_position, mut ball_linear_velocity) in
                    balls_query.iter_mut()
                {
                    if rigid_body.is_kinematic() {
                        ball_linear_velocity.0 = Vec2::ZERO;
                        ball_position.y = wall_animate.position.y - ROW_HEIGHT;
                    }
                }
            }
        }
    }
}
