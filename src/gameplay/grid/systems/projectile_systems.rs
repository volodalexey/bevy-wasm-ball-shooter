use bevy::prelude::{Commands, EventReader, EventWriter, Query, Res, ResMut, With};
use bevy_xpbd_2d::prelude::{AngularVelocity, LinearVelocity, Position, RigidBody};

use crate::gameplay::{
    ball::components::ProjectileBall,
    events::{FindCluster, MoveDownLastActive, ProjectileReload, SnapProjectile},
    grid::{
        resources::Grid,
        utils::{confine_grid_ball_position, convert_to_kinematic},
    },
    panels::resources::TurnCounter,
};

pub fn on_snap_projectile(
    mut snap_projectile_events: EventReader<SnapProjectile>,
    mut commands: Commands,
    grid: Res<Grid>,
    mut projectile_reload_writer: EventWriter<ProjectileReload>,
    mut turn_counter: ResMut<TurnCounter>,
    mut writer_find_cluster: EventWriter<FindCluster>,
    mut projectile_query: Query<
        (
            &mut ProjectileBall,
            &Position,
            &mut RigidBody,
            &mut LinearVelocity,
            &mut AngularVelocity,
        ),
        With<ProjectileBall>,
    >,
    mut writer_move_down_last_active: EventWriter<MoveDownLastActive>,
) {
    for SnapProjectile { projectile_entity } in snap_projectile_events.iter() {
        if let Ok((
            mut projectile_ball,
            projectile_position,
            mut rigid_body,
            mut linear_velocity,
            mut angular_velocity,
        )) = projectile_query.get_mut(*projectile_entity)
        {
            println!("SnapProjectile process {:?}", projectile_entity);
            // projectile ball can be removed by cluster and never snapped
            if projectile_ball.is_snapped {
                println!("Skip projectile {:?} already snapped", projectile_entity);
                continue;
            }
            projectile_ball.is_snapped = true;

            commands
                .entity(*projectile_entity)
                .remove::<ProjectileBall>();
            println!(
                "removed ProjectileBall from {:?} position y {}",
                projectile_entity, projectile_position.y
            );

            if let Some((confined_position, _, confined_y)) = confine_grid_ball_position(
                grid.as_ref(),
                projectile_entity,
                projectile_position.0,
                true,
            ) {
                if confined_y {
                    convert_to_kinematic(
                        &mut commands,
                        &projectile_entity,
                        rigid_body.as_mut(),
                        confined_position,
                        linear_velocity.as_mut(),
                        angular_velocity.as_mut(),
                    );
                }
            }

            turn_counter.0 += 1;

            println!("send ProjectileReload {:?}", projectile_entity);
            projectile_reload_writer.send(ProjectileReload);
            writer_find_cluster.send(FindCluster {
                to_check: vec![*projectile_entity],
            });
            writer_move_down_last_active.send(MoveDownLastActive {});
        }
    }
}
