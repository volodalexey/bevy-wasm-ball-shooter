use bevy::{
    prelude::{Entity, Event},
    utils::HashSet,
};

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
pub struct MoveDownLastActive;

#[derive(Event)]
pub struct SpawnRow;

#[derive(Event)]
pub struct FindCluster {
    pub to_check: HashSet<Entity>,
}
