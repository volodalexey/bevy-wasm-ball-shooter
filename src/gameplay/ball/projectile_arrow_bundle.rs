use bevy::prelude::{shape, Assets, Bundle, Mesh, PbrBundle, Res, ResMut, Transform, Vec3};

use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::ProjectileArrow;

#[derive(Bundle)]
pub struct ProjectileArrowBundle {
    pub pbr: PbrBundle,
    pub arrow: ProjectileArrow,
}

impl ProjectileArrowBundle {
    pub fn new(
        pos: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 1.0,
                    height: 10.0,
                    resolution: 10,
                    segments: 10,
                })),
                material: gameplay_materials.arrow.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            arrow: ProjectileArrow,
        }
    }
}
