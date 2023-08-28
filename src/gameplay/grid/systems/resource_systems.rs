use bevy::{
    prelude::{Entity, Input, KeyCode, Query, Res, ResMut, Vec2, With},
    utils::{HashMap, HashSet},
};
use bevy_xpbd_2d::prelude::{Position, RigidBody};

use crate::gameplay::{
    ball::components::{GridBall, ProjectileBall, Species},
    constants::LOG_KEYCODE_RESOURCES,
    grid::{resources::Grid, utils::build_entities_to_neighbours},
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
    keyboard_input_key_code: Res<Input<KeyCode>>,
) {
    let total = balls_query.iter().len();
    let mut entities: HashSet<Entity> = HashSet::with_capacity(total);
    let mut top_kinematic_position: f32 = f32::MIN;
    let mut entities_to_positions: HashMap<Entity, Vec2> = HashMap::with_capacity(total);
    let mut entities_to_species: HashMap<Entity, Species> = HashMap::with_capacity(total);
    balls_query
        .iter()
        .for_each(|(entity, position, species, grid_ball, rigid_body, _)| {
            if !grid_ball.is_ready_to_despawn {
                entities.insert(entity);
                entities_to_positions.insert(entity, position.0);
                entities_to_species.insert(entity, *species);
                if rigid_body.is_kinematic() {
                    top_kinematic_position = top_kinematic_position.max(position.0.y);
                }
            }
        });
    let entities_to_neighbours = build_entities_to_neighbours(&entities, &entities_to_positions);
    grid.entities_to_positions = entities_to_positions;
    grid.entities_to_species = entities_to_species;
    grid.entities_to_neighbours = entities_to_neighbours;
    grid.top_kinematic_position = top_kinematic_position;
    if keyboard_input_key_code.any_pressed([LOG_KEYCODE_RESOURCES]) {
        println!(
            "entities_to_positions {:?}\nentities_to_neighbours {:?}",
            grid.entities_to_positions, grid.entities_to_neighbours
        );
    }
}
