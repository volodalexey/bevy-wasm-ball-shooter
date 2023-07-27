use bevy::prelude::{default, PbrBundle, Res, Transform, Vec3};

use crate::gameplay::{materials::resources::GameplayMaterials, meshes::resources::GameplayMeshes};

use super::components::{OutBall, Species};

pub struct OutBallBundle;

impl OutBallBundle {
    pub fn new(
        pos: Vec3,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> (PbrBundle, OutBall, Species) {
        (
            PbrBundle {
                mesh: gameplay_meshes.grid_ball.clone(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(pos),
                ..default()
            },
            OutBall::default(),
            species,
        )
    }
}
