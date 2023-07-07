use bevy::prelude::Component;

#[derive(Component, Clone, Default)]
pub struct Projectile;

#[derive(Component)]
pub struct Flying(pub bool);
