use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform, Vec3, With,
};
use hexx::shapes;

use crate::gameplay::{
    ball::{components::Species, grid_ball_bundle::GridBallBundle},
    materials::resources::GameplayMaterials,
    meshes::resources::GameplayMeshes,
};

use super::{components::HexComponent, events::UpdatePositions, resources::Grid};

pub fn generate_grid(
    mut commands: Commands,
    gameplay_meshes: Option<Res<GameplayMeshes>>,
    gameplay_materials: Option<Res<GameplayMaterials>>,
    mut grid: ResMut<Grid>,
    mut update_positions: EventWriter<UpdatePositions>,
) {
    if let Some(gameplay_meshes) = gameplay_meshes {
        if let Some(gameplay_materials) = gameplay_materials {
            for hex in shapes::pointy_rectangle([0, grid.init_cols - 1, 0, grid.init_rows - 1]) {
                let (x, z) = grid.layout.hex_to_world_pos(hex).into();

                let entity = commands
                    .spawn((
                        GridBallBundle::new(
                            Vec3::new(x, 0.0, z),
                            grid.layout.hex_size.x,
                            Species::random_species(),
                            &gameplay_meshes,
                            &gameplay_materials,
                        ),
                        HexComponent { hex },
                    ))
                    .id();

                grid.set(hex, entity);
            }

            // Center grid on x-axis.
            grid.update_bounds();
            let (width, _) = grid.dim();
            grid.layout.origin.x = -width / 2. + grid.layout.hex_size.x;
            update_positions.send(UpdatePositions);
        }
    }
}

pub fn update_hex_coord_transforms(
    mut hexes: Query<(Entity, &mut Transform, &HexComponent), With<HexComponent>>,
    mut grid: ResMut<Grid>,
    mut event_query: EventReader<UpdatePositions>,
) {
    if event_query.is_empty() {
        return;
    }
    event_query.clear();

    grid.update_bounds();
    if grid.bounds.mins.y.abs() > grid.layout.hex_size.y.abs() {
        grid.layout.origin.y += grid.bounds.mins.y.abs() - grid.layout.hex_size.y.abs();
    }

    for (entity, mut transform, HexComponent { hex }) in hexes.iter_mut() {
        let hex = *hex;
        grid.set(hex, entity);
        let (x, z) = grid.layout.hex_to_world_pos(hex).into();
        transform.translation.x = x;
        transform.translation.z = z;
    }
}

pub fn cleanup_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    hexes: Query<Entity, With<HexComponent>>,
) {
    for entity in hexes.iter() {
        commands.entity(entity).despawn();
    }
    grid.clear();
}
