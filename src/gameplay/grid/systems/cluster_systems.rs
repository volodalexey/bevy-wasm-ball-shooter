use bevy::{
    prelude::{
        Assets, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, Input, KeyCode,
        Query, Res, ResMut, With,
    },
    sprite::ColorMaterial,
    time::Time,
    utils::HashSet,
};
use bevy_xpbd_2d::prelude::{LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::{
        components::{GridBall, ProjectileBall, Species},
        out_ball_bundle::OutBallBundle,
    },
    constants::{LOG_KEYCODE_CLUSTER, MIN_CLUSTER_SIZE},
    events::{FindCluster, ProjectileReload, UpdateScoreCounter},
    grid::{
        resources::{ClusterCheckCooldown, CollisionSnapCooldown, Grid},
        utils::find_cluster,
    },
    meshes::resources::GameplayMeshes,
    panels::resources::TurnCounter,
};

pub fn find_and_remove_clusters(
    mut commands: Commands,
    mut find_cluster_events: EventReader<FindCluster>,
    mut balls_query: Query<
        (
            Entity,
            &Position,
            &LinearVelocity,
            &Species,
            &mut GridBall,
            &mut RigidBody,
            Option<&ProjectileBall>,
        ),
        With<GridBall>,
    >,
    gameplay_meshes: Res<GameplayMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut writer_update_cooldown_counter: EventWriter<UpdateScoreCounter>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
    mut turn_counter: ResMut<TurnCounter>,
    mut projectile_reload_writer: EventWriter<ProjectileReload>,
    grid: Res<Grid>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut cluster_check_cooldown: ResMut<ClusterCheckCooldown>,
) {
    let is_paused = cluster_check_cooldown.timer.paused();
    if !is_paused {
        cluster_check_cooldown.timer.tick(time.delta());
    }
    let mut flush_checks = false;
    for FindCluster { to_check } in find_cluster_events.iter() {
        cluster_check_cooldown.to_check.insert(*to_check);
        if is_paused {
            flush_checks = true;
            cluster_check_cooldown.timer.unpause();
        }
    }

    if cluster_check_cooldown.timer.just_finished() {
        flush_checks = true;
        cluster_check_cooldown.timer.reset();
        cluster_check_cooldown.timer.pause();
    }

    if flush_checks {
        for start_from in cluster_check_cooldown.to_check.iter() {
            let (cluster, _) = find_cluster(
                *start_from,
                &grid.entities_to_neighbours,
                &grid.entities_to_species,
            );
            if keyboard_input_key_code.any_pressed([LOG_KEYCODE_CLUSTER]) {
                println!("cluster {:?}", cluster);
            }

            let mut cluster_score_add = 0;
            if cluster.len() >= MIN_CLUSTER_SIZE {
                // remove matching cluster
                cluster.iter().for_each(|cluster_entity| {
                    if let Ok((
                        cluster_entity,
                        cluster_position,
                        _,
                        cluster_species,
                        mut grid_ball,
                        _,
                        some_projectile_ball,
                    )) = balls_query.get_mut(*cluster_entity)
                    {
                        if !grid_ball.is_ready_to_despawn {
                            grid_ball.is_ready_to_despawn = true;
                            commands.spawn(OutBallBundle::new(
                                cluster_position.0,
                                *cluster_species,
                                &gameplay_meshes,
                                &mut materials,
                                false,
                            ));
                            println!("cluster entity despawned {:?}", cluster_entity);
                            commands.entity(cluster_entity).despawn_recursive();
                            cluster_score_add += 1;
                            if some_projectile_ball.is_some() {
                                println!("projectile removed in cluster {:?}", cluster_entity);
                                turn_counter.0 += 1;
                                collision_snap_cooldown.stop();
                                projectile_reload_writer.send(ProjectileReload);
                            }
                        }
                    }
                });
            }

            writer_update_cooldown_counter.send(UpdateScoreCounter {
                score_add: cluster_score_add,
            });
        }
        cluster_check_cooldown.to_check = HashSet::default();
    }
}
