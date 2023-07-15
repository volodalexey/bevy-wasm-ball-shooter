use bevy::prelude::{Entity, Event, Vec3};
use hexx::Hex;

use crate::gameplay::ball::Species;

#[derive(Event)]
pub struct SnapProjectile {
    /// Entity of the ball if any were hit.
    pub entity: Option<Entity>,
    /// Hit normal outwards from the projectile if any ball were hit.
    pub hit_normal: Option<Vec3>,
}

#[derive(Event)]
pub struct SpawnedBall {
    pub hex: Hex,
    pub species: Species,
}
