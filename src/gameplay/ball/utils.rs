use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, With};

use super::components::{AimLine, NextProjectileBall};

pub fn cleanup_next_projectile_ball_utils(
    commands: &mut Commands,
    query: &Query<Entity, With<NextProjectileBall>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_aim_line_utils(
    commands: &mut Commands,
    aim_line_query: &Query<Entity, With<AimLine>>,
) {
    for projectile_entity in aim_line_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}
