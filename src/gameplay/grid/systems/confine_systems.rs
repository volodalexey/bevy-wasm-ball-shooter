use bevy::prelude::{info, Entity, EventWriter, Query, Res, ResMut, With};
use bevy_xpbd_2d::prelude::Position;
use hexx::Hex;

use crate::gameplay::{
    ball::components::ProjectileBall,
    events::SnapProjectile,
    grid::{
        resources::{CollisionSnapCooldown, Grid},
        utils::send_snap_projectile,
    },
};

pub fn confine_grid_balls(
    projectile_query: Query<(Entity, &Position, &ProjectileBall), With<ProjectileBall>>,
    grid: Res<Grid>,
    mut writer_snap_projectile: EventWriter<SnapProjectile>,
    mut collision_snap_cooldown: ResMut<CollisionSnapCooldown>,
) {
    for (projectile_entity, projectile_position, projectile_ball) in projectile_query.iter() {
        if !projectile_ball.is_flying {
            return;
        }
        let hex = Hex {
            x: 0,
            y: grid.last_active_row,
        };
        let position = grid.layout.hex_to_world_pos(hex);
        if projectile_position.y > position.y {
            info!(
                "Projectile {:?} out of grid snap {} {}",
                projectile_entity, position.y, projectile_position.y
            );
            send_snap_projectile(
                collision_snap_cooldown.as_mut(),
                &mut writer_snap_projectile,
                projectile_entity,
            );
        }
    }
}
