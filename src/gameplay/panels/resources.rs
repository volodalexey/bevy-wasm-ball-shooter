use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct TurnCounter(pub u32);

#[derive(Resource, Default)]
pub struct MoveDownCounter(pub u32);

#[derive(Resource, Default)]
pub struct SpawnRowsLeft(pub u32);

#[derive(Resource, Default)]
pub struct ScoreCounter(pub u32);
