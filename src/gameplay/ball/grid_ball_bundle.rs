use bevy::prelude::{Bundle, PbrBundle, Res, Transform, Vec3};
use bevy_rapier3d::prelude::{Collider, LockedAxes, RigidBody};

use crate::gameplay::{materials::resources::GameplayMaterials, meshes::resources::GameplayMeshes};

use super::{
    components::{GridBall, Species},
    constants::INNER_RADIUS_COEFF,
};

#[derive(Bundle)]
pub struct GridBallBundle {
    pub pbr: PbrBundle,
    pub ball: GridBall,
    pub species: Species,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
}

impl GridBallBundle {
    pub fn new(
        pos: Vec3,
        radius: f32,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: gameplay_meshes.grid_ball.clone(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            ball: GridBall,
            species,
            collider: Collider::ball(radius * INNER_RADIUS_COEFF),
            rigid_body: RigidBody::Fixed,
            locked_axes: LockedAxes::TRANSLATION_LOCKED_Z,
        }
    }
}
