use bevy::prelude::{Entity, Input, KeyCode, Query, Res, Vec2, With, Without};
use bevy_xpbd_2d::prelude::{ExternalForce, LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, GridBallScaleAnimate, MagneticGridBall, ProjectileBall},
    constants::{
        LOG_KEYCODE_MAGNETIC, MAGNETIC_DISTANCE_STRONG, MAGNETIC_DISTANCE_WEAK,
        MAGNETIC_FACTOR_STRONG, MAGNETIC_FACTOR_WEAK, MAX_GRID_BALL_SPEED,
    },
    grid::resources::Grid,
};

pub fn apply_magnetic_forces(
    mut magnetic_balls_query: Query<
        (
            Entity,
            &Position,
            &GridBall,
            &mut ExternalForce,
            &mut LinearVelocity,
            Option<&GridBallScaleAnimate>,
            &RigidBody,
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
            some_grid_ball_animate_scale,
            rigid_body,
        )) = magnetic_balls_query.get_mut(*entity)
        {
            if some_grid_ball_animate_scale.is_some() || rigid_body.is_kinematic() {
                // other entities can attract to this but this can not attract to other
                continue;
            }

            let mut result_acc_strong = Vec2::ZERO;
            let mut result_acc_weak = Vec2::ZERO;

            for (neighbour, distance) in neighbours.iter() {
                let is_strong_range = *distance < MAGNETIC_DISTANCE_STRONG;
                let is_weak_range = *distance < MAGNETIC_DISTANCE_WEAK;
                if is_strong_range || is_weak_range {
                    if let Some(neighbour_position) = grid.entities_to_positions.get(neighbour) {
                        let direction = *neighbour_position - position.0;
                        if is_strong_range {
                            result_acc_strong += direction;
                        } else {
                            result_acc_weak += direction;
                        }
                    }
                } else {
                    break;
                }
            }
            let result_strong_normilized = result_acc_strong.normalize_or_zero();
            let result_weak_normilized = result_acc_weak.normalize_or_zero();
            let result_magnetic_force = result_strong_normilized * MAGNETIC_FACTOR_STRONG
                + result_weak_normilized * MAGNETIC_FACTOR_WEAK;
            external_force.set_force(result_magnetic_force);
            linear_velocity.0 = linear_velocity.0.clamp_length_max(MAX_GRID_BALL_SPEED);

            if keyboard_input_key_code.any_just_pressed([LOG_KEYCODE_MAGNETIC]) {
                println!("applied magnetic to entity {:?} result_strong_normilized {} result_weak_normilized {}", entity, result_strong_normilized, result_weak_normilized);
            }
        }
    }
}
