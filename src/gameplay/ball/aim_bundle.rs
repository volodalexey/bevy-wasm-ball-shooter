use bevy::{
    prelude::{shape, Assets, Bundle, Mesh, Quat, Res, ResMut, Transform, Vec2, Vec3, Visibility},
    sprite::MaterialMesh2dBundle,
};

use crate::gameplay::{
    constants::{AIM_LINE_Z_INDEX, AIM_TARGET_Z_INDEX, BALL_DIAMETER, INNER_RADIUS_COEFF},
    grid::resources::Grid,
    materials::resources::GameplayMaterials,
};

use super::{components::AimLine, components::AimTarget};

pub struct AimBundle;

impl AimBundle {
    pub fn new_target(
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
        grid: &Res<Grid>,
    ) -> impl Bundle {
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(grid.layout.hex_size.y * INNER_RADIUS_COEFF).into())
                    .into(),
                material: gameplay_materials.aim_target.clone(),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, AIM_TARGET_Z_INDEX)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            AimTarget::default(),
        )
    }

    pub fn new_line(
        from_pos: Vec2,
        to_pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        gameplay_materials: &Res<GameplayMaterials>,
    ) -> impl Bundle {
        let line_center = from_pos + (to_pos - from_pos) / 2.0;
        let distance = to_pos.distance(from_pos);
        let scale_y = distance;
        let diff_center = to_pos - line_center;
        let angle = diff_center.y.atan2(diff_center.x);
        let rotation = Quat::from_rotation_z(angle + core::f32::consts::PI / 2.0);
        (
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(BALL_DIAMETER, 1.0)).into())
                    .into(),
                material: gameplay_materials.aim_target.clone(),
                transform: Transform::from_translation(line_center.extend(AIM_LINE_Z_INDEX))
                    .with_scale(Vec3::new(1.0, scale_y, 1.0))
                    .with_rotation(rotation),
                ..Default::default()
            },
            AimLine,
        )
    }
}
