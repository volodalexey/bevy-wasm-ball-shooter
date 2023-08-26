use bevy::{
    prelude::{info, Entity, EventReader, EventWriter, Query, Res, ResMut, Vec2, With, Without},
    time::Time,
};
use bevy_xpbd_2d::prelude::{
    AngularVelocity, CollisionEnded, CollisionStarted, LinearVelocity, Position,
};

use crate::gameplay::{
    ball::components::{GridBall, GridBallScaleAnimate, ProjectileBall},
    constants::MIN_PROJECTILE_SNAP_DOT,
    events::{FindCluster, SnapProjectile},
    grid::{
        resources::{ClusterCheckCooldown, CollisionSnapCooldown},
        utils::{is_move_reverse, is_move_slow, send_snap_projectile},
    },
};

pub fn check_collision_events(
    mut collision_started_events: EventReader<CollisionStarted>,
    mut collision_ended_events: EventReader<CollisionEnded>,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
    mut projectile_query: Query<
        (
            Entity,
            &mut Position,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut ProjectileBall,
        ),
        With<ProjectileBall>,
    >,
    balls_query: Query<
        (Entity, &Position),
        (
            With<GridBall>,
            Without<ProjectileBall>,
            Without<GridBallScaleAnimate>,
        ),
    >,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    mut writer_find_cluster: EventWriter<FindCluster>,
    time: Res<Time>,
    mut cluster_check_cooldown: ResMut<ClusterCheckCooldown>,
) {
    for CollisionStarted(entity_a, entity_b) in collision_started_events.iter() {
        let result_ball_entity_a = balls_query.get(*entity_a);
        let result_ball_entity_b = balls_query.get(*entity_b);

        let mut result_projectile = projectile_query.get_mut(*entity_a);
        if result_projectile.is_err() {
            result_projectile = projectile_query.get_mut(*entity_b);
        }
        if result_ball_entity_a.is_ok() && result_ball_entity_b.is_ok() {
            cluster_check_cooldown.timer.tick(time.delta());
            cluster_check_cooldown
                .to_check
                .extend(vec![entity_a, entity_b]);
            if cluster_check_cooldown.timer.just_finished() || cluster_check_cooldown.is_ready() {
                writer_find_cluster.send(FindCluster {
                    to_check: cluster_check_cooldown
                        .to_check
                        .clone()
                        .into_iter()
                        .collect(),
                    move_down_after: false,
                });
                cluster_check_cooldown.restart();
            }
        } else if let Ok((_, ball_position)) = result_ball_entity_a.or(result_ball_entity_b) {
            if let Ok((
                projectile_entity,
                projectile_position,
                mut proj_linear_velocity,
                mut proj_angular_velocity,
                mut projectile_ball,
            )) = result_projectile
            {
                if projectile_ball.snap_vel == Vec2::ZERO {
                    let to_pos = ball_position.0;
                    let from_pos = projectile_position.0;
                    let diff = (to_pos - from_pos).normalize();
                    let vel = proj_linear_velocity.0.normalize();
                    let dot = vel.dot(diff);
                    if dot > MIN_PROJECTILE_SNAP_DOT {
                        collision_snap_cooldown.start();
                        // save first touch position
                        is_move_reverse(&mut projectile_ball, proj_linear_velocity.0);
                    }
                    proj_linear_velocity.0 = Vec2::ZERO;
                    proj_angular_velocity.0 = 0.0;
                }
                let is_move_slow_result = is_move_slow(proj_linear_velocity.0);
                let is_move_reverse_result =
                    is_move_reverse(&mut projectile_ball, proj_linear_velocity.0);
                if is_move_slow_result || is_move_reverse_result {
                    // if ball turned back
                    // or ball moves too slow
                    info!(
                        "Projectile {:?} too slow so snap on collision started",
                        projectile_entity
                    );
                    send_snap_projectile(
                        collision_snap_cooldown.as_mut(),
                        &mut writer_snap_projectile,
                        projectile_entity,
                    );
                }
            }
        }
    }
    for CollisionEnded(entity_a, entity_b) in collision_ended_events.iter() {
        if let Ok((_, _)) = balls_query.get(*entity_a).or(balls_query.get(*entity_b)) {
            let mut result_projectile = projectile_query.get_mut(*entity_a);
            if result_projectile.is_err() {
                result_projectile = projectile_query.get_mut(*entity_b);
            }
            if let Ok((projectile_entity, _, projectile_velocity, _, mut projectile_ball)) =
                result_projectile
            {
                let is_move_slow_result = is_move_slow(projectile_velocity.0);
                let is_move_reverse_result =
                    is_move_reverse(&mut projectile_ball, projectile_velocity.0);
                if is_move_slow_result || is_move_reverse_result {
                    // if ball turned back
                    // or ball moves too slow
                    info!(
                        "Projectile {:?} too slow so snap on collision ended",
                        projectile_entity
                    );
                    send_snap_projectile(
                        collision_snap_cooldown.as_mut(),
                        &mut writer_snap_projectile,
                        projectile_entity,
                    );
                }
            }
        }
    }
}

pub fn tick_collision_snap_cooldown_timer(
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    time: Res<Time>,
    mut projectile_query: Query<
        (Entity, &mut ProjectileBall, &LinearVelocity),
        With<ProjectileBall>,
    >,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
) {
    if !collision_snap_cooldown.timer.paused() {
        collision_snap_cooldown.timer.tick(time.delta());
        if let Ok((projectile_entity, mut projectile_ball, linear_velocity)) =
            projectile_query.get_single_mut()
        {
            if collision_snap_cooldown.is_ready_for_check(|| {
                println!("is_ready_for_check {}", linear_velocity.0.length());
                is_move_slow(linear_velocity.0)
                    || is_move_reverse(&mut projectile_ball, linear_velocity.0)
            }) {
                // snap projectile anyway after some time
                info!("Projectile {:?} cooldown snap", projectile_entity);
                send_snap_projectile(
                    collision_snap_cooldown.as_mut(),
                    &mut writer_snap_projectile,
                    projectile_entity,
                );
            }
        }
    }
}
