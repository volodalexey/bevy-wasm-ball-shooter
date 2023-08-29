use bevy::{
    prelude::{default, Bundle, Res, Transform, Vec2, Vec3},
    sprite::MaterialMesh2dBundle,
};

use crate::gameplay::{
    constants::NEXT_PROJECTILE_Z_INDEX, materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
};

use super::components::{NextProjectileBall, Species};

pub struct NextProjectileBallBundle;

impl NextProjectileBallBundle {
    pub fn new(
        pos: Vec2,
        species: Species,
        gameplay_meshes: &Res<GameplayMeshes>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.next_projectile_ball.clone().into(),
                material: gameplay_materials.from_species(species),
                transform: Transform::from_translation(Vec3::new(
                    pos.x,
                    pos.y,
                    NEXT_PROJECTILE_Z_INDEX,
                )),
                ..default()
            },
            NextProjectileBall {},
            species,
        )
    }
}
