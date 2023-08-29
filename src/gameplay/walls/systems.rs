use bevy::prelude::{
    Assets, Commands, DespawnRecursiveExt, Entity, Mesh, Query, Res, ResMut, Vec2, With,
};
use hexx::Hex;

use crate::gameplay::constants::{BALL_RADIUS, WALL_SIDE_WIDTH, WALL_TOP_HEIGHT};
use crate::gameplay::grid::resources::Grid;
use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::WallType;
use super::wall_bundle::WallBundle;

pub fn setup_level_walls(
    mut commands: Commands,
    grid: Res<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Option<Res<GameplayMaterials>>,
) {
    if let Some(gameplay_materials) = gameplay_materials {
        let inner_width = grid.init_cols as f32 * BALL_RADIUS;
        let side_x = inner_width + WALL_SIDE_WIDTH * 0.5;
        let top_hex = Hex::new(0, grid.last_active_row + 1);
        let position = grid.layout.hex_to_world_pos(top_hex);
        let top_y = position.y + BALL_RADIUS + WALL_TOP_HEIGHT / 2.0;
        WallBundle::spawn(
            &mut commands,
            Vec2::new(side_x, 0.0),
            WallType::Left,
            &mut meshes,
            &gameplay_materials,
        );
        WallBundle::spawn(
            &mut commands,
            Vec2::new(-side_x, 0.0),
            WallType::Right,
            &mut meshes,
            &gameplay_materials,
        );
        WallBundle::spawn(
            &mut commands,
            Vec2::new(0.0, top_y),
            WallType::Top,
            &mut meshes,
            &gameplay_materials,
        );
    }
}

pub fn cleanup_level_walls(mut commands: Commands, wall_query: Query<Entity, With<WallType>>) {
    for wall_entity in wall_query.iter() {
        commands.entity(wall_entity).despawn_recursive();
    }
}
