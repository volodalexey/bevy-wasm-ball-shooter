use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct ProjectileReload;

#[derive(Event)]
pub struct SnapProjectile {
    pub projectile_entity: Entity,
}

#[derive(Event)]
pub struct UpdateScoreCounter {
    pub score_add: u32,
}

#[derive(Event)]
pub struct MoveDownTopWall;

#[derive(Event)]
pub struct SpawnRow;

#[derive(Event)]
pub struct FindCluster {
    pub to_check: Entity,
}
