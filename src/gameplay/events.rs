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
    pub move_down_after: bool,
}

#[derive(Event)]
pub struct MoveDownLastActive;

#[derive(Event)]
pub struct SpawnRow;

#[derive(Event)]
pub struct FindCluster {
    pub to_check: Vec<Entity>,
    pub move_down_after: bool,
}
