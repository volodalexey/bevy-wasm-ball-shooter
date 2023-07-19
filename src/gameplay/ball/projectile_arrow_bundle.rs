use bevy::prelude::{
    shape, Assets, Bundle, Mesh, PbrBundle, Res, ResMut, Transform, Vec3, Visibility,
};

use crate::gameplay::materials::resources::GameplayMaterials;

use super::{components::ProjectileArrow, constants::INNER_RADIUS_COEFF};

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
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius: INNER_RADIUS_COEFF,
                        subdivisions: 1,
                    })
                    .expect("Unable to generate IcoSphere")
                    .into(),
                ),
                material: gameplay_materials.arrow_end.clone(),
                transform: Transform::from_translation(pos),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            arrow: ProjectileArrow,
        }
    }
}
