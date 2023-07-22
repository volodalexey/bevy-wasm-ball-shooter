use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TurnCounter(pub u32);

#[derive(Resource)]
pub struct MoveCounter(pub u32);

#[derive(Resource)]
pub struct ScoreCounter(pub u32);
