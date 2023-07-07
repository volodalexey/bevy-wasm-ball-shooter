use bevy::{asset::HandleId, prelude::Component};

#[derive(Component, Clone, Default)]
pub struct Projectile;

#[derive(Component)]
pub struct Flying(pub bool);

#[derive(Component)]
pub struct FlyLine {
    pub handle_id: HandleId,
}
