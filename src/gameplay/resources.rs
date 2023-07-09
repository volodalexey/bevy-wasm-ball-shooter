use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TurnCounter(pub u32);

#[derive(Resource)]
pub struct RoundTurnCounter(pub u32);

#[derive(Resource)]
pub struct Score(pub u32);
