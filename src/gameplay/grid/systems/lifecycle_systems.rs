use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, NextState, Query, Res, ResMut, Vec2,
        With,
    },
    utils::HashSet,
    window::{PrimaryWindow, Window},
};
use bevy_xpbd_2d::prelude::{AngularVelocity, LinearVelocity, Position, RigidBody};
use hexx::{shapes, Hex};

use crate::{
    components::AppState,
    gameplay::{
        ball::{
            components::{GridBall, OutBall},
            grid_ball_bundle::GridBallBundle,
        },
        constants::ROW_HEIGHT,
        events::SpawnRow,
        grid::{
            resources::{ClusterCheckCooldown, Grid},
            utils::adjust_grid_layout,
        },
        materials::resources::GameplayMaterials,
        meshes::resources::GameplayMeshes,
        panels::resources::SpawnRowsLeft,
        walls::components::TopWall,
    },
};

pub fn generate_grid(
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut grid: ResMut<Grid>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    adjust_grid_layout(&window_query, &mut grid, 0);
    let max_side_x = (grid.init_cols / 2) as i32;
    let min_col = -max_side_x;
    let max_col = max_side_x;
    let min_row = -(grid.total_rows as i32) + 1;
    let max_row = 0;
    for hex in shapes::pointy_rectangle([min_col, max_col, min_row, max_row]) {
        let is_even = hex.y % 2 == 0;
        let offset = hex.to_offset_coordinates(grid.offset_mode);
        if (!is_even && offset[0] == max_side_x) || hex.y < grid.last_active_row {
            continue;
        }
        let is_last_active = hex.y == grid.last_active_row;
        let position = grid.layout.hex_to_world_pos(hex);

        GridBallBundle::spawn(
            &mut commands,
            &gameplay_meshes,
            &gameplay_materials,
            grid.total_colors,
            position,
            is_last_active,
            false,
            None,
            true,
            true,
        );
    }
    app_state_next_state.set(AppState::Gameplay);
}

pub fn cleanup_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    grid_balls_query: Query<Entity, With<GridBall>>,
    out_balls_query: Query<Entity, With<OutBall>>,
    mut cluster_check_cooldown: ResMut<ClusterCheckCooldown>,
) {
    for entity in grid_balls_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in out_balls_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    cluster_check_cooldown.timer.reset();
    cluster_check_cooldown.to_check = HashSet::default();
    grid.clear();
}

pub fn spawn_new_row(
    mut commands: Commands,
    gameplay_meshes: Res<GameplayMeshes>,
    gameplay_materials: Res<GameplayMaterials>,
    mut spawn_row_events: EventReader<SpawnRow>,
    mut grid: ResMut<Grid>,
    mut grid_balls_query: Query<
        (
            Entity,
            &GridBall,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut RigidBody,
        ),
        With<RigidBody>,
    >,
    mut spawn_rows_left: ResMut<SpawnRowsLeft>,
    mut top_wall_query: Query<&mut Position, With<TopWall>>,
) {
    if spawn_row_events.is_empty() {
        return;
    }
    spawn_row_events.clear();

    if spawn_rows_left.0 > 0 {
        spawn_rows_left.0 -= 1;
        grid.last_active_row -= 1;

        for mut position in top_wall_query.iter_mut() {
            position.y = position.y + ROW_HEIGHT; // jump to release place for next row
        }

        let max_side_x = (grid.init_cols / 2) as i32;
        for hex_x in -max_side_x..=max_side_x {
            let is_even = grid.last_active_row % 2 == 0;
            let hex = Hex::from_offset_coordinates([hex_x, grid.last_active_row], grid.offset_mode);
            let offset = hex.to_offset_coordinates(grid.offset_mode);
            if (!is_even && offset[0] == max_side_x) || hex.y < grid.last_active_row {
                continue;
            }
            let position = grid.layout.hex_to_world_pos(hex);
            GridBallBundle::spawn(
                &mut commands,
                &gameplay_meshes,
                &gameplay_materials,
                grid.total_colors,
                position,
                true,
                false,
                None,
                true,
                true,
            );
        }

        for (entity, grid_ball, mut velocity, mut angular_velocity, mut rigid_body) in
            grid_balls_query.iter_mut()
        {
            if !grid_ball.is_ready_to_despawn && rigid_body.is_kinematic() {
                *rigid_body = RigidBody::Dynamic;
                velocity.0 = Vec2::ZERO;
                angular_velocity.0 = 0.0;
                println!("Converted to dynamic {:?}", entity);
            }
        }
    }
}
