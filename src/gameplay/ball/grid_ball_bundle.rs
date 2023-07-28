use bevy::prelude::{default, PbrBundle, Res, Transform, Vec3};
use bevy_rapier3d::prelude::{Collider, RigidBody};
use hexx::Hex;

use crate::gameplay::{materials::resources::GameplayMaterials, meshes::resources::GameplayMeshes};

use super::{
    components::{GridBall, Species},
    constants::INNER_RADIUS_COEFF,
};

pub struct GridBallBundle;

impl GridBallBundle {
    pub fn new(
        pos: Vec3,
        radius: f32,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
        hex: Hex,
    ) -> (PbrBundle, GridBall, Species, RigidBody, Collider) {
        (
            PbrBundle {
                mesh: gameplay_meshes.grid_ball.clone(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(pos),
                ..default()
            },
            GridBall { hex },
            species,
            RigidBody::KinematicPositionBased,
            Collider::ball(radius * INNER_RADIUS_COEFF),
        )
    }
}
