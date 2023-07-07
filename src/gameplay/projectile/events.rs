use bevy::prelude::{Entity, Vec3};

use crate::gameplay::{ball::Species, hex::Coord};

pub struct SnapProjectile {
    /// Entity of the ball if any were hit.
    pub entity: Option<Entity>,
    /// Hit normal outwards from the projectile if any ball were hit.
    pub hit_normal: Option<Vec3>,
}

pub struct SpawnedBall {
    pub hex: Coord,
    pub species: Species,
}
