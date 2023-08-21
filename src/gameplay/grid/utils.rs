use bevy::{
    prelude::{default, ChildBuilder, Color, Entity, EventWriter, Query, Vec2, With},
    text::{Text, Text2dBounds, Text2dBundle, TextSection, TextStyle},
    utils::{HashMap, HashSet},
    window::{PrimaryWindow, Window},
};
use hexx::Hex;

use crate::gameplay::{
    ball::components::{ProjectileBall, Species},
    constants::{
        BALL_DIAMETER, CELL_SIZE, CLUSTER_TOLERANCE, EMPTY_PLAYGROUND_HEIGHT,
        MIN_PROJECTILE_SNAP_VELOCITY, PROJECTILE_SPAWN_BOTTOM, ROW_HEIGHT,
    },
    events::SnapProjectile,
    panels::resources::MoveCounter,
};

use super::resources::{CollisionSnapCooldown, Grid};

pub fn buid_cells_to_entities(
    entities_to_positions: &HashMap<Entity, Vec2>,
) -> HashMap<(i32, i32), HashSet<Entity>> {
    let mut cells_to_entities: HashMap<(i32, i32), HashSet<Entity>> = HashMap::default();
    for (entity, position) in entities_to_positions.iter() {
        // https://leetless.de/posts/spatial-hashing-vs-ecs/
        // generate storage of balls by cells
        let cell_index_x: i32 = (position.x / CELL_SIZE).floor() as i32;
        let cell_index_y: i32 = (position.y / CELL_SIZE).floor() as i32;
        let key = (cell_index_x, cell_index_y);
        cells_to_entities
            .entry(key)
            .or_insert(default())
            .insert(*entity);
    }
    cells_to_entities
}

pub fn build_entities_to_neighbours<'a>(
    entities_to_positions: &HashMap<Entity, Vec2>,
    cells_to_entities: &HashMap<(i32, i32), HashSet<Entity>>,
) -> HashMap<Entity, HashSet<Entity>> {
    let mut entities_to_neighbours: HashMap<Entity, HashSet<Entity>> = HashMap::default();
    for (entity, position) in entities_to_positions.iter() {
        // generate storage of connections
        let cell_index_x: i32 = (position.x / CELL_SIZE).floor() as i32;
        let cell_index_y: i32 = (position.y / CELL_SIZE).floor() as i32;
        for cell_y in cell_index_y - 1..=cell_index_y + 1 {
            for cell_x in cell_index_x - 1..=cell_index_x + 1 {
                let key = (cell_x, cell_y);
                if let Some(neighbours) = cells_to_entities.get(&key) {
                    for neighbour_entity in neighbours {
                        if let Some(neighbour_position) =
                            entities_to_positions.get(neighbour_entity)
                        {
                            if neighbour_entity.index() != entity.index()
                                && position.distance(*neighbour_position) < CLUSTER_TOLERANCE
                            {
                                entities_to_neighbours
                                    .entry(*entity)
                                    .or_insert(default())
                                    .insert(*neighbour_entity);
                            }
                        }
                    }
                }
            }
        }
    }
    entities_to_neighbours
}

pub fn find_cluster(
    start_from: Entity,
    entities_to_neighbours: &HashMap<Entity, HashSet<Entity>>,
    entities_to_species: &HashMap<Entity, Species>,
) -> (HashSet<Entity>, HashSet<Entity>) {
    let mut to_process: Vec<&Entity> = vec![&start_from];
    let mut processed: HashSet<Entity> = HashSet::default();
    let mut cluster: HashSet<Entity> = HashSet::default();

    while let Some(current) = to_process.pop() {
        // find clusters with the same color
        if processed.contains(current) {
            continue;
        }
        cluster.insert(*current);
        processed.insert(*current);
        if let Some(current_species) = entities_to_species.get(current) {
            if let Some(neighbours) = entities_to_neighbours.get(current) {
                for neighbour in neighbours.iter() {
                    // if neighbour is still in the grid and wasn't removed by cluster
                    if processed.contains(neighbour) {
                        continue;
                    }
                    if let Some(neighbour_species) = entities_to_species.get(neighbour) {
                        if current_species != neighbour_species {
                            continue;
                        }
                        to_process.push(neighbour);
                    }
                }
            }
        }
    }
    (cluster, processed)
}

// pub fn find_floating_clusters(
//     removed_cluster: &HashSet<Entity>,
//     entities_to_neighbours: &HashMap<Entity, HashSet<Entity>>,
//     last_active_entities: &HashSet<Entity>,
// ) -> Vec<HashSet<Entity>> {
//     let mut processed: HashSet<Entity> = HashSet::default();
//     let mut floating_clusters: Vec<HashSet<Entity>> = vec![];

//     for entity in removed_cluster.iter() {
//         if processed.contains(entity) {
//             continue;
//         }
//         if last_active_entities.contains(entity) {
//             processed.insert(*entity);
//             continue;
//         }

//         let mut to_process: Vec<&Entity> = vec![entity];
//         let mut processed: HashSet<Entity> = HashSet::default();
//         let mut any_cluster: HashSet<Entity> = HashSet::default();
//         let mut found_floating = false;
//         while let Some(current) = to_process.pop() {
//             if processed.contains(current) {
//                 continue;
//             }
//             any_cluster.insert(*current);
//             processed.insert(*current);
//             if let Some(neighbours) = entities_to_neighbours.get(current) {
//                 for neighbour in neighbours.iter() {
//                     // if neighbour is still in the grid and wasn't removed by cluster
//                     if processed.contains(neighbour) {
//                         continue;
//                     }
//                     to_process.push(neighbour);
//                     if last_active_entities.contains(neighbour) {
//                         found_floating = true;
//                         break;
//                     }
//                 }
//             }
//         }

//         if found_floating {
//             continue;
//         } else if any_cluster.len() > 0 {
//             floating_clusters.push(any_cluster);
//         }
//     }
//     floating_clusters
// }

pub fn adjust_grid_layout(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    grid: &mut Grid,
    move_counter: &MoveCounter,
) {
    let window = window_query.single();
    let spawn_bottom_world_y = -(window.height() - PROJECTILE_SPAWN_BOTTOM - window.height() / 2.0);
    let init_layout_y = spawn_bottom_world_y + EMPTY_PLAYGROUND_HEIGHT;
    let move_layout_y = move_counter.0 as f32 * ROW_HEIGHT;
    grid.layout.origin.y = init_layout_y - move_layout_y;
    println!("grid.layout.origin.y {}", grid.layout.origin.y);
}

pub fn is_move_slow(velocity: Vec2) -> bool {
    velocity.length() <= MIN_PROJECTILE_SNAP_VELOCITY
}

/// detect that projectile after snap is moving into the same clockwise/counterclockwise direction around snap grid ball
pub fn is_move_reverse(projectile_ball: &mut ProjectileBall, projectile_velocity: Vec2) -> bool {
    if projectile_ball.snap_vel == Vec2::ZERO {
        projectile_ball.snap_vel = projectile_velocity.normalize();
    } else {
        let dot = projectile_ball
            .snap_vel
            .dot(projectile_velocity.normalize());
        if dot < 0.0 {
            return true;
        }
    }
    false
}

pub fn build_ball_text(parent: &mut ChildBuilder<'_, '_, '_>, some_hex: Option<Hex>) {
    let mut text_sections = vec![TextSection {
        value: format!("  {:?} ", parent.parent_entity()),
        style: TextStyle {
            color: Color::BLACK,
            ..default()
        },
    }];
    if let Some(hex) = some_hex {
        text_sections.push(TextSection {
            value: format!(" {},{}", hex.x, hex.y),
            style: TextStyle {
                color: Color::BLACK,
                ..default()
            },
        });
    }
    parent.spawn(Text2dBundle {
        text: Text {
            sections: text_sections,
            ..default()
        },
        text_2d_bounds: Text2dBounds {
            size: Vec2::new(BALL_DIAMETER, BALL_DIAMETER),
        },
        ..default()
    });
}

pub fn send_snap_projectile(
    collision_snap_cooldown: &mut CollisionSnapCooldown,
    writer_snap_projectile: &mut EventWriter<SnapProjectile>,
    projectile_entity: Entity,
) {
    collision_snap_cooldown.stop();
    writer_snap_projectile.send(SnapProjectile { projectile_entity });
}
