use bevy::{
    prelude::{default, ChildBuilder, Color, Commands, Entity, EventWriter, Query, Vec2, With},
    text::{Text, Text2dBounds, Text2dBundle, TextSection, TextStyle},
    utils::{HashMap, HashSet},
    window::{PrimaryWindow, Window},
};
use bevy_xpbd_2d::prelude::{AngularVelocity, LinearVelocity, RigidBody};
use hexx::Hex;

use crate::gameplay::{
    ball::components::{GridBallPositionAnimate, ProjectileBall, Species},
    constants::{
        BALL_DIAMETER, EMPTY_PLAYGROUND_HEIGHT, LOCK_POSITION_TOLERANCE,
        MIN_PROJECTILE_REVERSE_VELOCITY, MIN_PROJECTILE_SNAP_VELOCITY,
        NEIGHBOUR_POSITION_TOLERANCE, PROJECTILE_SPAWN_BOTTOM, ROW_HEIGHT,
    },
    events::SnapProjectile,
    panels::resources::MoveCounter,
};

use super::resources::{CollisionSnapCooldown, Grid};

pub fn build_entities_to_neighbours<'a>(
    entities: &HashSet<Entity>,
    entities_to_positions: &HashMap<Entity, Vec2>,
) -> HashMap<Entity, Vec<(Entity, f32)>> {
    let mut entities_to_neighbours: HashMap<Entity, Vec<(Entity, f32)>> =
        HashMap::with_capacity(entities.len());
    for entity in entities.iter() {
        for neighbour_entity in entities.iter() {
            if neighbour_entity != entity {
                if let Some(entity_position) = entities_to_positions.get(entity) {
                    let entry = entities_to_neighbours
                        .entry(*entity)
                        .or_insert(Vec::with_capacity(entities_to_positions.len()));

                    if let Some(neighbour_position) = entities_to_positions.get(neighbour_entity) {
                        entry.push((
                            *neighbour_entity,
                            (*entity_position).distance(*neighbour_position),
                        ));
                    }
                }
            }
        }
    }
    for (_, neighbours) in entities_to_neighbours.iter_mut() {
        neighbours.sort_by(|(_, distance_a), (_, distance_b)| distance_a.total_cmp(distance_b));
    }
    entities_to_neighbours
}

pub fn find_cluster(
    start_from: Entity,
    entities_to_neighbours: &HashMap<Entity, Vec<(Entity, f32)>>,
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
                for (neighbour, distance) in neighbours.iter() {
                    // if neighbour is still in the grid and wasn't removed by cluster
                    if processed.contains(neighbour) {
                        continue;
                    }
                    if *distance > NEIGHBOUR_POSITION_TOLERANCE {
                        break;
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
    println!("Adjust Grid Layout y {}", grid.layout.origin.y);
}

pub fn is_move_slow(velocity: Vec2) -> bool {
    velocity.length() <= MIN_PROJECTILE_SNAP_VELOCITY
}

/// detect that projectile after snap is moving into the same clockwise/counterclockwise direction around snap grid ball
pub fn is_move_reverse(projectile_ball: &mut ProjectileBall, projectile_velocity: Vec2) -> bool {
    if projectile_ball.snap_vel == Vec2::ZERO {
        projectile_ball.snap_vel = projectile_velocity.normalize();
    } else {
        if projectile_velocity.y < MIN_PROJECTILE_REVERSE_VELOCITY {
            return true;
        }
        let dot = projectile_ball
            .snap_vel
            .dot(projectile_velocity.normalize());
        println!("dot {}", dot);
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

pub fn confine_grid_ball_position(
    grid: &Grid,
    entity: &Entity,
    entity_position: Vec2,
    strict_check: bool,
) -> Option<(Vec2, bool, bool)> {
    if grid.top_kinematic_position == f32::MIN {
        return None;
    }
    let max_side_x = grid.init_cols / 2;
    let snap_hex = grid.layout.world_pos_to_hex(entity_position);
    let mut offset = snap_hex.to_offset_coordinates(grid.offset_mode);
    let mut confined_x = false;
    let mut confined_y = false;
    let min_col = -(max_side_x as i32);
    let last_kinematic_row = grid
        .layout
        .world_pos_to_hex(Vec2::new(0.0, grid.top_kinematic_position))
        .to_offset_coordinates(grid.offset_mode);
    if (strict_check && offset[1] <= last_kinematic_row[1]) || offset[1] < last_kinematic_row[1] {
        offset[1] = last_kinematic_row[1];
        confined_y = true;
    }
    let max_col = match offset[1] % 2 == 0 {
        true => max_side_x,
        false => max_side_x - 1,
    } as i32;
    if (strict_check && offset[0] <= min_col) || offset[0] < min_col {
        offset[0] = min_col;
        confined_x = true;
    }
    if (strict_check && offset[0] >= max_col) || offset[0] > max_col {
        offset[0] = max_col;
        confined_x = true;
    }

    if confined_x || confined_y {
        let corrected_hex = Hex::from_offset_coordinates(offset, grid.offset_mode);
        let all_neighbours = corrected_hex.all_neighbors();
        let possible_neighbours = all_neighbours.iter().filter(|hex| {
            let neighbour_offset = hex.to_offset_coordinates(grid.offset_mode);
            neighbour_offset[0] >= min_col
                && neighbour_offset[0] <= max_col
                && neighbour_offset[1] >= last_kinematic_row[1]
        });
        let mut possible_positions: Vec<Vec2> = vec![grid.layout.hex_to_world_pos(corrected_hex)];
        for possible_neighbour in possible_neighbours {
            possible_positions.push(grid.layout.hex_to_world_pos(*possible_neighbour));
        }

        // check that corrected position is free
        for (ball_entity, ball_position) in grid.entities_to_positions.iter() {
            if ball_entity == entity {
                continue;
            }

            let index = 0;
            loop {
                if let Some(check_position) = possible_positions.get(index) {
                    if ball_position.y - LOCK_POSITION_TOLERANCE <= check_position.y
                        && check_position.y <= ball_position.y + LOCK_POSITION_TOLERANCE
                        && ball_position.x - LOCK_POSITION_TOLERANCE <= check_position.x
                        && check_position.x <= ball_position.x + LOCK_POSITION_TOLERANCE
                    {
                        possible_positions.remove(index);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if let Some(possible_position) = possible_positions.first() {
            return Some((*possible_position, confined_x, confined_y));
        }
    }
    None
}

pub fn convert_to_kinematic(
    commands: &mut Commands,
    entity: &Entity,
    rigid_body: &mut RigidBody,
    snap_position: Vec2,
    linear_velocity: &mut LinearVelocity,
    angular_velocity: &mut AngularVelocity,
) {
    *rigid_body = RigidBody::Kinematic;
    commands
        .entity(*entity)
        .insert(GridBallPositionAnimate::from_position(snap_position, false));
    linear_velocity.0 = Vec2::ZERO;
    angular_velocity.0 = 0.0;
}
