use bevy::prelude::{
    Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform,
    Vec2, Vec3, With,
};
use hexx::shapes;

use crate::{
    gameplay::{
        ball::{
            components::{ProjectileBall, Species},
            events::SnapProjectile,
            grid_ball_bundle::GridBallBundle,
        },
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        ui::resources::MoveCounter,
    },
    resources::LevelCounter,
};

use super::{
    components::HexComponent, events::UpdatePositions, resources::Grid, utils::adjust_grid_layout,
};

pub fn generate_grid(
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut grid: ResMut<Grid>,
    mut update_positions: EventWriter<UpdatePositions>,
    level_counter: Res<LevelCounter>,
) {
    let factor: i32 = (level_counter.0 * 2) as i32;
    grid.init_cols = factor.clamp(2, 16);
    grid.init_rows = factor;
    for hex in shapes::pointy_rectangle([0, grid.init_cols - 1, 0, grid.init_rows - 1]) {
        let (x, z) = grid.layout.hex_to_world_pos(hex).into();

        let entity = commands
            .spawn((
                GridBallBundle::new(
                    Vec3::new(x, 0.0, z),
                    grid.layout.hex_size.x,
                    Species::random_species(&level_counter),
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
    adjust_grid_layout(&mut grid, &MoveCounter(0));
    update_positions.send(UpdatePositions);
}

pub const VISIBLE_ROWS: f32 = 5.0;

pub fn update_hex_coord_transforms(
    mut hexes: Query<(Entity, &mut Transform, &HexComponent), With<HexComponent>>,
    mut grid: ResMut<Grid>,
    mut event_query: EventReader<UpdatePositions>,
    move_counter: Res<MoveCounter>,
) {
    if event_query.is_empty() {
        return;
    }
    event_query.clear();

    adjust_grid_layout(&mut grid, &move_counter);
    grid.update_bounds();

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

pub fn check_projectile_out_of_grid(
    mut commands: Commands,
    mut projectile_query: Query<
        (Entity, &Transform, &ProjectileBall, &Species),
        With<ProjectileBall>,
    >,
    mut grid: ResMut<Grid>,
    mut snap_projectile: EventWriter<SnapProjectile>,
) {
    if let Ok((projectile_entity, projectile_transform, projectile_ball, species)) =
        projectile_query.get_single_mut()
    {
        if !projectile_ball.is_flying {
            return;
        }
        if grid.bounds.dirty {
            grid.update_bounds();
        }
        if projectile_transform.translation.z < grid.bounds.mins.y + grid.layout.hex_size.y {
            commands.entity(projectile_entity).despawn_recursive();
            snap_projectile.send(SnapProjectile {
                out_of_bounds: true,
                pos: Vec2::new(
                    projectile_transform.translation.x,
                    projectile_transform.translation.z,
                ),
                species: *species,
            });
        }
    }
}
