use bevy::{
    prelude::{shape, Assets, Mesh, Res, ResMut, Transform, Vec2, Vec3, Visibility},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::gameplay::{grid::resources::Grid, materials::resources::GameplayMaterials};

use super::{components::AimLine, components::AimTarget, constants::INNER_RADIUS_COEFF};

pub struct AimBundle;

impl AimBundle {
    pub fn new_target(
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
        grid: &Res<Grid>,
    ) -> (MaterialMesh2dBundle<ColorMaterial>, AimTarget) {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(grid.layout.hex_size.y * INNER_RADIUS_COEFF).into())
                    .into(),
                material: gameplay_materials.arrow_end.clone(),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            AimTarget,
        )
    }

    pub fn new_line(
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> (MaterialMesh2dBundle<ColorMaterial>, AimLine) {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                    .into(),
                material: gameplay_materials.arrow_end.clone(),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            AimLine,
        )
    }
}
