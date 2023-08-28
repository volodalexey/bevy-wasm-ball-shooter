use bevy::prelude::{
    info, Commands, Entity, EventWriter, Input, KeyCode, Query, Res, ResMut, With, Without,
};
use bevy_xpbd_2d::prelude::{AngularVelocity, LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, ProjectileBall},
    constants::{LOG_KEYCODE_CONFINE, LOG_KEYCODE_CONFINE_ALL, MAX_GRID_BALL_SPEED},
    events::SnapProjectile,
    grid::{
        resources::{CollisionSnapCooldown, Grid},
        utils::{confine_grid_ball_position, convert_to_kinematic, send_snap_projectile},
    },
};

pub fn confine_grid_balls(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Position, &ProjectileBall), With<ProjectileBall>>,
    grid: Res<Grid>,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    mut balls_query: Query<
        (
            Entity,
            &Position,
            &GridBall,
            &mut RigidBody,
            &mut LinearVelocity,
            &mut AngularVelocity,
        ),
        (With<GridBall>, Without<ProjectileBall>),
    >,
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    for (projectile_entity, projectile_position, projectile_ball) in projectile_query.iter() {
        if !projectile_ball.is_flying {
            break;
        }

        if projectile_position.y > grid.top_kinematic_position {
            info!(
                "Projectile {:?} out of grid snap {} < {}",
                projectile_entity, grid.top_kinematic_position, projectile_position.y
            );
            send_snap_projectile(
                collision_snap_cooldown.as_mut(),
                &mut writer_snap_projectile,
                projectile_entity,
            );
        }
    }
    for (entity, position, grid_ball, mut rigid_body, mut linear_velocity, mut angular_velocity) in
        balls_query.iter_mut()
    {
        if keyboard_input_key_code.any_just_pressed([LOG_KEYCODE_CONFINE_ALL]) {
            println!(
                "Try Confine {:?} is_ready_to_despawn {} rigid_body({}|{})",
                entity,
                grid_ball.is_ready_to_despawn,
                rigid_body.is_dynamic(),
                rigid_body.is_kinematic()
            );
        }
        if !grid_ball.is_ready_to_despawn && rigid_body.is_dynamic() {
            linear_velocity.0 = linear_velocity.0.clamp_length_max(MAX_GRID_BALL_SPEED);
            if let Some((confined_position, _, confined_y)) = confine_grid_ball_position(
                &grid,
                &entity,
                position.0,
                true,
                keyboard_input_key_code.any_just_pressed([LOG_KEYCODE_CONFINE]),
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
                    if keyboard_input_key_code.any_just_pressed([LOG_KEYCODE_CONFINE]) {
                        println!("Confined {:?}", entity);
                    }
                }
            }
        }
    }
}
