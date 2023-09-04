use bevy::prelude::{
    Assets, ColorMaterial, Commands, DespawnRecursiveExt, Entity, Handle, Query, ResMut, Transform,
    Vec2, With,
};
use bevy_xpbd_2d::prelude::{ExternalForce, LinearVelocity};

use crate::gameplay::{
    ball::components::{OutBall, OutBallAnimation},
    constants::OUT_BALL_GRAVITY,
};

pub fn animate_out_ball(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut balls_query: Query<
        (
            &mut OutBall,
            &mut Transform,
            &mut LinearVelocity,
            &mut ExternalForce,
            &Handle<ColorMaterial>,
        ),
        With<OutBall>,
    >,
) {
    for (
        mut grid_ball_out,
        mut ball_transform,
        mut linear_velocity,
        mut external_force,
        ball_material,
    ) in balls_query.iter_mut()
    {
        if !grid_ball_out.started {
            grid_ball_out.started = true;
            ball_transform.translation.z = 2.0; // slightly on top of grid
            if grid_ball_out.animation_type == OutBallAnimation::FloatingCluster {
                linear_velocity.0 = Vec2::new(0.0, fastrand::i32(-200..=0) as f32);
            } else {
                linear_velocity.0 = Vec2::new(
                    match fastrand::bool() {
                        true => fastrand::i32(-200..=-100) as f32,
                        false => fastrand::i32(100..=200) as f32,
                    },
                    fastrand::i32(100..=200) as f32,
                );
            }
            external_force.set_force(Vec2::new(0.0, -OUT_BALL_GRAVITY));
        } else {
            if let Some(ball_material) = materials.get_mut(&ball_material) {
                ball_material.color.set_a(ball_material.color.a() - 0.01);
                if ball_material.color.a() <= 0.0 {
                    grid_ball_out.marked_for_delete = true;
                }
            }
        }
    }
}

pub fn check_out_ball_for_delete(
    mut commands: Commands,
    out_balls_query: Query<(Entity, &OutBall), With<OutBall>>,
) {
    for (ball_entity, grid_ball_out) in out_balls_query.iter() {
        if grid_ball_out.marked_for_delete {
            commands.entity(ball_entity).despawn_recursive();
        }
    }
}
