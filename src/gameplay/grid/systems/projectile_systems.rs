use bevy::prelude::{Commands, EventReader, EventWriter, Query, ResMut, With};
use bevy_xpbd_2d::prelude::Position;

use crate::gameplay::{
    ball::components::ProjectileBall,
    events::{FindCluster, MoveDownTopWall, ProjectileReload, SnapProjectile},
    panels::resources::TurnCounter,
};

pub fn on_snap_projectile(
    mut snap_projectile_events: EventReader<SnapProjectile>,
    mut commands: Commands,
    mut projectile_reload_writer: EventWriter<ProjectileReload>,
    mut turn_counter: ResMut<TurnCounter>,
    mut writer_find_cluster: EventWriter<FindCluster>,
    mut projectile_query: Query<(&mut ProjectileBall, &Position), With<ProjectileBall>>,
    mut writer_move_down_last_active: EventWriter<MoveDownTopWall>,
) {
    for SnapProjectile { projectile_entity } in snap_projectile_events.iter() {
        if let Ok((mut projectile_ball, projectile_position)) =
            projectile_query.get_mut(*projectile_entity)
        {
            // projectile ball can be removed by cluster and never snapped
            if projectile_ball.is_snapped {
                println!(
                    "Skip projectile {:?} already snapped {:?}",
                    projectile_entity, projectile_position.0
                );
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

            turn_counter.0 += 1;

            projectile_reload_writer.send(ProjectileReload);
            writer_find_cluster.send(FindCluster {
                to_check: *projectile_entity,
            });
            writer_move_down_last_active.send(MoveDownTopWall);
        }
    }
}
