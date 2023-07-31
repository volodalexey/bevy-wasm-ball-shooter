use bevy::prelude::{
    Assets, Commands, DespawnRecursiveExt, Entity, Mesh, Query, Res, ResMut, Vec3, With,
};

use crate::gameplay::ball::constants::INNER_RADIUS_COEFF;
use crate::gameplay::constants::PROJECTILE_SPAWN;
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
        let ball_radius = grid.layout.hex_size.x * INNER_RADIUS_COEFF;
        let inner_width = grid.init_cols as f32 * ball_radius + ball_radius * 0.5;
        let side_x = inner_width + WALL_X_WIDTH * 0.5;
        let bottom = PROJECTILE_SPAWN + grid.layout.hex_size.y;
        commands.spawn(WallBundle::new(
            Vec3::new(side_x, 0.0, 0.0),
            WallType::Left,
            &mut meshes,
            &gameplay_materials,
            bottom * 2.0,
        ));
        commands.spawn(WallBundle::new(
            Vec3::new(-side_x, 0.0, 0.0),
            WallType::Right,
            &mut meshes,
            &gameplay_materials,
            bottom * 2.0,
        ));
    }
}

pub fn cleanup_level_walls(mut commands: Commands, wall_query: Query<Entity, With<WallType>>) {
    for wall_entity in wall_query.iter() {
        commands.entity(wall_entity).despawn_recursive();
    }
}
