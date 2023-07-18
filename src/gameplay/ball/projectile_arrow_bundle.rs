use bevy::prelude::{
    shape, Assets, Bundle, Mesh, PbrBundle, Quat, Res, ResMut, Transform, Vec3, Visibility,
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
                mesh: meshes
                    .add(shape::Circle::new(INNER_RADIUS_COEFF).into())
                    .into(),
                material: gameplay_materials.arrow.clone(),
                transform: Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_x(-core::f32::consts::PI / 2.0)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            arrow: ProjectileArrow,
        }
    }
}
