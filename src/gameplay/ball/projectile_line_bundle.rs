use bevy::prelude::{
    default, shape, Assets, Bundle, Mesh, PbrBundle, Quat, Res, ResMut, Transform, Vec3, Visibility,
};

use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::{ProjectileLine, ProjectileLineParent};

#[derive(Bundle)]
pub struct ProjectileLineBundle {
    pub pbr: PbrBundle,
    pub line: ProjectileLine,
}

const PROJECTILE_LINE_WIDTH: f32 = 0.2;

impl ProjectileLineBundle {
    pub fn new_parent(pos: Vec3) -> (PbrBundle, ProjectileLineParent) {
        (
            PbrBundle {
                transform: Transform::from_translation(pos),
                visibility: Visibility::Hidden,
                ..default()
            },
            ProjectileLineParent,
        )
    }

    pub fn new_child(
        pos: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: PROJECTILE_LINE_WIDTH / 2.0,
                    ..default()
                })),
                material: gameplay_materials.arrow_line.clone(),
                transform: Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_x(-core::f32::consts::PI / 2.0)),
                ..Default::default()
            },
            line: ProjectileLine,
        }
    }
}
