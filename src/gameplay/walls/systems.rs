use bevy::prelude::{
    Assets, Commands, DespawnRecursiveExt, Entity, Mesh, Query, Res, ResMut, Vec3, With,
};

use crate::gameplay::ball::constants::INNER_RADIUS_COEFF;
use crate::gameplay::grid::resources::Grid;
use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::WallType;
use super::wall_bundle::{WallBundle, WALL_X_WIDTH};

pub fn setup_level_walls(
    mut commands: Commands,
    grid: Res<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Option<Res<GameplayMaterials>>,
) {
    if let Some(gameplay_materials) = gameplay_materials {
        let side_x = grid.init_cols as f32 * grid.layout.hex_size.x * INNER_RADIUS_COEFF
            + grid.layout.hex_size.x * INNER_RADIUS_COEFF * 0.5
            + WALL_X_WIDTH * 0.5;
        commands.spawn(WallBundle::new(
            Vec3::new(side_x, 0.0, 0.0),
            WallType::Left,
            &mut meshes,
            &gameplay_materials,
        ));
        commands.spawn(WallBundle::new(
            Vec3::new(-side_x, 0.0, 0.0),
            WallType::Right,
            &mut meshes,
            &gameplay_materials,
        ));
    }
}

pub fn cleanup_level_walls(mut commands: Commands, wall_query: Query<Entity, With<WallType>>) {
    for wall_entity in wall_query.iter() {
        commands.entity(wall_entity).despawn_recursive();
    }
}
