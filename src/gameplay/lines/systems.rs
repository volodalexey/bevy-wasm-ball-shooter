use bevy::prelude::{
    Assets, Commands, DespawnRecursiveExt, Entity, Mesh, Query, Res, ResMut, With,
};

use crate::gameplay::constants::{BALL_DIAMETER, BALL_RADIUS};
use crate::gameplay::grid::resources::Grid;
use crate::gameplay::materials::resources::GameplayMaterials;

use super::components::LineType;
use super::line_bundle::LineBundle;

pub fn setup_level_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_materials: Option<Res<GameplayMaterials>>,
    grid: Res<Grid>,
) {
    if let Some(gameplay_materials) = gameplay_materials {
        let width = (grid.init_cols as f32 * BALL_DIAMETER + BALL_RADIUS) * 2.0;
        commands.spawn(LineBundle::new(
            width,
            LineType::GridTop,
            &mut meshes,
            &gameplay_materials,
        ));
        commands.spawn(LineBundle::new(
            width,
            LineType::GameOver,
            &mut meshes,
            &gameplay_materials,
        ));
    }
}

pub fn cleanup_level_lines(mut commands: Commands, lines_query: Query<Entity, With<LineType>>) {
    for line_entity in lines_query.iter() {
        commands.entity(line_entity).despawn_recursive();
    }
}
