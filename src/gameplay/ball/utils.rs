use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, With};

use super::components::NextProjectileBall;

pub fn cleanup_next_projectile_ball_utils(
    commands: &mut Commands,
    query: &Query<Entity, With<NextProjectileBall>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
