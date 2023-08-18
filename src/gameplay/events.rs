use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct ProjectileReload;

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
    pub start_from: Entity,
}

#[derive(Event)]
pub struct CheckJoints {
    pub a: Entity,
    pub b: Entity,
}
