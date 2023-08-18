use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct BeginTurn;

#[derive(Event)]
pub struct UpdateScoreCounter {
    pub score_add: u32,
}

#[derive(Event)]
pub struct UpdateMoveDown;

#[derive(Event)]
pub struct SpawnRow;

#[derive(Event)]
pub struct FindCluster {
    pub start_from: Entity,
}

#[derive(Event)]
pub struct CheckJoints {
    pub start_from: Entity,
}
