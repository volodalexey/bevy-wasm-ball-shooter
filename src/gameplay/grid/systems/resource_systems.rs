use bevy::{
    prelude::{Entity, Query, ResMut, Vec2, With},
    utils::HashMap,
};
use bevy_xpbd_2d::prelude::{Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, ProjectileBall, Species},
    grid::{
        resources::Grid,
        utils::{buid_cells_to_entities, build_entities_to_neighbours},
    },
};

pub fn update_grid_resources(
    mut grid: ResMut<Grid>,
    balls_query: Query<
        (
            Entity,
            &Position,
            &Species,
            &mut GridBall,
            &mut RigidBody,
            Option<&ProjectileBall>,
        ),
        With<GridBall>,
    >,
) {
    let total = balls_query.iter().len();
    let mut entities_to_positions: HashMap<Entity, Vec2> = HashMap::with_capacity(total);
    let mut entities_to_species: HashMap<Entity, Species> = HashMap::with_capacity(total);
    balls_query.iter().for_each(|(e, position, sp, gb, _, _)| {
        if !gb.is_ready_to_despawn {
            entities_to_positions.insert(e, position.0);
            entities_to_species.insert(e, *sp);
        }
    });
    let cells_to_entities = buid_cells_to_entities(&entities_to_positions);
    let entities_to_neighbours =
        build_entities_to_neighbours(&entities_to_positions, &cells_to_entities);
    grid.entities_to_positions = entities_to_positions;
    grid.entities_to_species = entities_to_species;
    grid.cells_to_entities = cells_to_entities;
    grid.entities_to_neighbours = entities_to_neighbours;
}
