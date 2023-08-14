use bevy::prelude::Event;

#[derive(Event)]
pub struct BeginTurn;

#[derive(Event)]
pub struct UpdateCooldownCounter;

#[derive(Event)]
pub struct UpdateMoveDown;

#[derive(Event)]
pub struct SpawnRow;
