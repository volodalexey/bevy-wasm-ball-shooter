use std::collections::HashMap;

use bevy::prelude::{
    Assets, Changed, Color, Commands, Entity, Mesh, Query, Res, ResMut, StandardMaterial,
    Transform, Vec3, With,
};
use bevy_prototype_debug_lines::DebugLines;

use crate::gameplay::{
    ball::{random_species, BallBundle},
    hex::{rectangle, Coord, Direction},
};

use super::resources::Grid;

pub fn move_down_and_spawn(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: &mut Grid,
) {
    let mut hash_map: HashMap<Coord, Option<&Entity>> = HashMap::new();
    for (&hex, entity) in grid.storage.iter() {
        let dir = match grid.layout.is_pointy() {
            true => match hex.r % 2 == 0 {
                true => Direction::F,
                false => Direction::E,
            },
            false => Direction::F,
        };

        let down = hex.neighbor(dir);
        commands.entity(*entity).insert(down);
        hash_map.insert(down, Some(entity));
    }

    grid.storage = hash_map
        .iter()
        .map(|(&hex, &entity)| (hex, entity.unwrap().clone()))
        .collect();

    for hex in rectangle(grid.columns(), 1, &grid.layout) {
        let world_pos = grid.layout.to_world_y(hex, 0.0);
        let ball = commands
            .spawn((
                BallBundle::new(
                    world_pos,
                    grid.layout.size.x,
                    random_species(),
                    &mut meshes,
                    &mut materials,
                ),
                hex,
            ))
            .id();

        grid.set(hex, Some(ball));
    }
}

pub fn generate_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<Grid>,
    hexes: Query<Entity, With<Coord>>,
) {
    for entity in hexes.iter() {
        commands.entity(entity).despawn();
    }

    grid.clear();

    const WIDTH: i32 = 16;
    const HEIGHT: i32 = 16;

    for hex in rectangle(WIDTH, HEIGHT, &grid.layout) {
        let world_pos = grid.layout.to_world_y(hex, 0.0);
        let entity = commands
            .spawn((
                BallBundle::new(
                    world_pos,
                    grid.layout.size.x,
                    random_species(),
                    &mut meshes,
                    &mut materials,
                ),
                hex,
            ))
            .id();

        grid.set(hex, Some(entity));
    }

    grid.update_bounds();

    // Center grid on x-axis.
    let (width, _) = grid.dim();
    grid.layout.origin.x = -width / 2.;

    grid.update_bounds();
}

pub fn update_hex_coord_transforms(
    mut hexes: Query<(Entity, &mut Transform, &Coord), Changed<Coord>>,
    mut grid: ResMut<Grid>,
) {
    for (entity, mut transform, hex) in hexes.iter_mut() {
        grid.set(*hex, Some(entity));
        let (x, z) = grid.layout.to_world(*hex).into();
        transform.translation.x = x;
        transform.translation.z = z;
    }
}

pub fn display_grid_bounds(grid: Res<Grid>, mut lines: ResMut<DebugLines>) {
    const Z_LENGTH: f32 = 1000.;

    lines.line_colored(
        Vec3::new(grid.bounds.mins.x, 0., Z_LENGTH),
        Vec3::new(grid.bounds.mins.x, 0., -Z_LENGTH),
        0.,
        Color::GRAY,
    );

    lines.line_colored(
        Vec3::new(grid.bounds.maxs.x, 0., Z_LENGTH),
        Vec3::new(grid.bounds.maxs.x, 0., -Z_LENGTH),
        0.,
        Color::GRAY,
    );
}

pub fn cleanup_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    hexes: Query<Entity, With<Coord>>,
) {
    for entity in hexes.iter() {
        commands.entity(entity).despawn();
    }
    grid.clear();
}
