use bevy::prelude::{Entity, Input, KeyCode, Query, Res, Vec2, With, Without};
use bevy_xpbd_2d::prelude::{ExternalForce, Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, GridBallScaleAnimate, MagneticGridBall, ProjectileBall},
    constants::{
        LOG_KEYCODE_MAGNETIC, MAGNETIC_DISTANCE_STRONG, MAGNETIC_FACTOR_STRONG,
        MAGNETIC_FACTOR_WEAK,
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
            some_grid_ball_animate_scale,
            rigid_body,
        )) = magnetic_balls_query.get_mut(*entity)
        {
            if some_grid_ball_animate_scale.is_some() || rigid_body.is_kinematic() {
                // other entities can attract to this but this can not attract to other
                continue;
            }

            let mut result_acc_strong = Vec2::ZERO;
            let mut strong_count: Vec<Entity> = vec![];
            let mut result_acc_weak = Vec2::ZERO;
            let mut weak_count: Vec<Entity> = vec![];

            for (neighbour, distance) in neighbours.iter() {
                let is_strong_range = *distance < MAGNETIC_DISTANCE_STRONG;
                if let Some(neighbour_position) = grid.entities_to_positions.get(neighbour) {
                    let direction = *neighbour_position - position.0;
                    if is_strong_range {
                        result_acc_strong += direction;
                        strong_count.push(*neighbour);
                    } else {
                        result_acc_weak += direction;
                        weak_count.push(*neighbour);
                    }
                }
            }
            let result_strong_normilized = result_acc_strong.normalize_or_zero();
            let result_weak_normilized = result_acc_weak.normalize_or_zero();
            let result_magnetic_force = result_strong_normilized * MAGNETIC_FACTOR_STRONG
                + result_weak_normilized * MAGNETIC_FACTOR_WEAK;
            external_force.set_force(result_magnetic_force);

            if keyboard_input_key_code.any_just_pressed([LOG_KEYCODE_MAGNETIC]) {
                println!("applied magnetic to entity {:?} result_strong_normilized {} result_weak_normilized {} strong_count {:?} weak_count {:?}", 
                entity, result_strong_normilized, result_weak_normilized, strong_count, weak_count);
            }
        }
    }
}
