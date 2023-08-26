use bevy::prelude::{Commands, Entity, Input, KeyCode, Query, Res, Vec2, With, Without};
use bevy_xpbd_2d::prelude::{AngularVelocity, ExternalForce, LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, GridBallScaleAnimate, MagneticGridBall, ProjectileBall},
    constants::{
        MAGNETIC_DISTANCE_STRONG, MAGNETIC_DISTANCE_WEAK, MAGNETIC_FACTOR_STRONG,
        MAGNETIC_FACTOR_WEAK, MAX_GRID_BALL_SPEED,
    },
    grid::{
        resources::Grid,
        utils::{confine_grid_ball_position, convert_to_kinematic},
    },
};

pub fn apply_magnetic_forces(
    mut commands: Commands,
    mut magnetic_balls_query: Query<
        (
            Entity,
            &Position,
            &GridBall,
            &mut ExternalForce,
            &mut LinearVelocity,
            &mut AngularVelocity,
            Option<&GridBallScaleAnimate>,
            &mut RigidBody,
        ),
        (With<MagneticGridBall>, Without<ProjectileBall>),
    >,
    grid: Res<Grid>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    for (entity, neighbours) in grid.entities_to_neighbours.iter() {
        if let Ok((
            entity,
            position,
            _,
            mut external_force,
            mut linear_velocity,
            mut angular_velocity,
            some_grid_ball_animate_scale,
            mut rigid_body,
        )) = magnetic_balls_query.get_mut(*entity)
        {
            if some_grid_ball_animate_scale.is_some() || rigid_body.is_kinematic() {
                // other entities can attract to this but this can not attract to other
                continue;
            }
            let mut result_acc_strong = Vec2::ZERO;
            let mut result_acc_weak = Vec2::ZERO;
            for neighbour in neighbours.iter() {
                if let Some(neighbour_position) = grid.entities_to_positions.get(neighbour) {
                    let direction = *neighbour_position - position.0;
                    let dist = position.distance(*neighbour_position);
                    if dist < MAGNETIC_DISTANCE_STRONG {
                        result_acc_strong += direction;
                    } else if dist < MAGNETIC_DISTANCE_WEAK {
                        result_acc_weak += direction;
                    }
                }
            }
            let result_strong_normilized = result_acc_strong.normalize_or_zero();
            let result_weak_normilized = result_acc_weak.normalize_or_zero();
            let result_magnetic_force = result_strong_normilized * MAGNETIC_FACTOR_STRONG
                + result_weak_normilized * MAGNETIC_FACTOR_WEAK;
            external_force.set_force(result_magnetic_force);
            linear_velocity.0 = linear_velocity.0.clamp_length_max(MAX_GRID_BALL_SPEED);
            if keyboard_input_key_code.any_just_released([KeyCode::L]) {
                println!("applied magnetic to entity {:?} result_strong_normilized {} result_weak_normilized {}", entity, result_strong_normilized, result_weak_normilized);
            }
            if let Some((confined_position, _, confined_y)) = confine_grid_ball_position(
                &grid.entities_to_positions,
                &grid,
                &entity,
                position.0,
                true,
            ) {
                if confined_y {
                    convert_to_kinematic(
                        &mut commands,
                        &entity,
                        rigid_body.as_mut(),
                        confined_position,
                        linear_velocity.as_mut(),
                        angular_velocity.as_mut(),
                    );
                }
            }
        }
    }
}
