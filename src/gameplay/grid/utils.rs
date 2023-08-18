use bevy::{
    prelude::{
        default, BuildChildren, ChildBuilder, Color, Commands, DespawnRecursiveExt, Entity, Query,
        Transform, Vec2, With,
    },
    text::{Text, Text2dBounds, Text2dBundle, TextSection, TextStyle},
    utils::{HashMap, HashSet},
    window::{PrimaryWindow, Window},
};
use bevy_rapier2d::prelude::{ImpulseJoint, PrismaticJointBuilder, RevoluteJointBuilder};
use hexx::Hex;

use crate::gameplay::{
    ball::components::{GridBall, LastActiveGridBall, ProjectileBall, Species},
    constants::{
        BALL_DIAMETER, BALL_RADIUS, BUILD_JOINT_TOLERANCE, CELL_SIZE, CLUSTER_TOLERANCE,
        EMPTY_PLAYGROUND_HEIGHT, MIN_PROJECTILE_SNAP_VELOCITY, PROJECTILE_SPAWN_BOTTOM, ROW_HEIGHT,
    },
    panels::resources::MoveCounter,
};

use super::resources::Grid;

pub fn buid_cell_storage(
    balls_query: &Query<
        (
            Entity,
            &Transform,
            &Species,
            &mut GridBall,
            Option<&LastActiveGridBall>,
        ),
        With<GridBall>,
    >,
) -> HashMap<(i32, i32), Vec<(Entity, Vec2, Species, bool)>> {
    let mut cell_storage: HashMap<(i32, i32), Vec<(Entity, Vec2, Species, bool)>> =
        HashMap::default();
    for (ball_entity, ball_transform, ball_species, grid_ball, ball_last_active) in
        balls_query.iter()
    {
        if grid_ball.is_ready_to_despawn {
            continue;
        }
        // https://leetless.de/posts/spatial-hashing-vs-ecs/
        // generate storage of balls by cells
        let ball_position = ball_transform.translation.truncate();
        let cell_index_x: i32 = (ball_position.x / CELL_SIZE).floor() as i32;
        let cell_index_y: i32 = (ball_position.y / CELL_SIZE).floor() as i32;
        let key = (cell_index_x, cell_index_y);
        cell_storage.entry(key).or_insert(default()).push((
            ball_entity,
            ball_position,
            *ball_species,
            match ball_last_active {
                Some(_) => true,
                None => false,
            },
        ));
    }
    cell_storage
}

pub fn build_connection_storage(
    balls_query: &Query<
        (
            Entity,
            &Transform,
            &Species,
            &mut GridBall,
            Option<&LastActiveGridBall>,
        ),
        With<GridBall>,
    >,
    cell_storage: &HashMap<(i32, i32), Vec<(Entity, Vec2, Species, bool)>>,
) -> HashMap<Entity, (Species, bool, Vec<(Entity, Vec2, Species)>)> {
    let mut connection_storage: HashMap<Entity, (Species, bool, Vec<(Entity, Vec2, Species)>)> =
        HashMap::default();
    for (ball_entity, ball_transform, ball_species, grid_ball, ball_last_active) in
        balls_query.iter()
    {
        if grid_ball.is_ready_to_despawn {
            continue;
        }
        // generate storage of connections
        let ball_position = ball_transform.translation.truncate();
        let cell_index_x: i32 = (ball_position.x / CELL_SIZE).floor() as i32;
        let cell_index_y: i32 = (ball_position.y / CELL_SIZE).floor() as i32;
        for cell_y in cell_index_y - 1..=cell_index_y + 1 {
            for cell_x in cell_index_x - 1..=cell_index_x + 1 {
                let key = (cell_x, cell_y);
                if let Some(neighbours) = cell_storage.get(&key) {
                    for (neighbour_entity, neighbour_position, neighbour_species, _) in neighbours {
                        if neighbour_entity.index() != ball_entity.index()
                            && ball_position.distance(*neighbour_position) < CLUSTER_TOLERANCE
                        {
                            connection_storage
                                .entry(ball_entity)
                                .or_insert((
                                    *ball_species,
                                    match ball_last_active {
                                        Some(_) => true,
                                        None => false,
                                    },
                                    default(),
                                ))
                                .2
                                .push((*neighbour_entity, *neighbour_position, *neighbour_species));
                        }
                    }
                }
            }
        }
    }
    connection_storage
}

pub fn find_cluster(
    start_from: Entity,
    connection_storage: &HashMap<Entity, (Species, bool, Vec<(Entity, Vec2, Species)>)>,
    check_species: bool,
) -> (Vec<(Entity, bool)>, HashSet<Entity>) {
    let mut to_process: Vec<&Entity> = vec![&start_from];
    let mut processed: HashSet<Entity> = HashSet::default();
    let mut cluster: Vec<(Entity, bool)> = vec![];

    while let Some(current) = to_process.pop() {
        // find clusters with the same color
        if let Some((current_species, last_active, neighbours)) = connection_storage.get(current) {
            if processed.contains(current) {
                continue;
            }
            cluster.push((*current, *last_active));
            processed.insert(*current);
            for (neighbour_entity, _, neighbour_species) in neighbours.iter() {
                if let Some(_) = connection_storage.get(neighbour_entity) {
                    // if neighbour is still in the grid and wasn't removed by cluster
                    if processed.contains(neighbour_entity) {
                        continue;
                    }
                    if check_species && current_species != neighbour_species {
                        continue;
                    }
                    to_process.push(neighbour_entity);
                }
            }
        }
    }
    (cluster, processed)
}

pub fn find_floating_clusters(
    connection_storage: &HashMap<Entity, (Species, bool, Vec<(Entity, Vec2, Species)>)>,
) -> Vec<Vec<(Entity, bool)>> {
    let mut processed: HashSet<Entity> = HashSet::default();
    let mut floating_clusters: Vec<Vec<(Entity, bool)>> = vec![];

    for (storage_entity, (_, storage_last_active, _)) in connection_storage.iter() {
        if processed.contains(storage_entity) {
            continue;
        }
        if *storage_last_active {
            processed.insert(*storage_entity);
            continue;
        }

        let (cluster, _processed) = find_cluster(*storage_entity, connection_storage, false);

        processed.extend(_processed);

        if cluster.len() <= 0 {
            continue;
        }

        let mut floating = true;
        for (_, cluster_last_active) in cluster.iter() {
            if *cluster_last_active {
                floating = false;
                break;
            }
        }
        if floating {
            floating_clusters.push(cluster);
        }
    }
    floating_clusters
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
}

pub fn build_revolute_joint(
    anchor_entity: &Entity,
    anchor_pos: Vec2,
    from_pos: Vec2,
    normalize: bool,
) -> ImpulseJoint {
    let diff = anchor_pos - from_pos;
    let axis = match normalize {
        true => diff.normalize() * BALL_DIAMETER,
        false => diff,
    };
    // println!(
    //     "from_pos({}, {}) to_pos({}, {}) diff({}, {}) axis({}, {})",
    //     from_pos.x, from_pos.y, anchor_pos.x, anchor_pos.y, diff.x, diff.y, axis.x, axis.y
    // );
    let joint = RevoluteJointBuilder::new().local_anchor2(axis);
    ImpulseJoint::new(*anchor_entity, joint)
}

pub fn build_prismatic_joint(from_pos: Vec2, to_pos: Vec2, to_entity: Entity) -> ImpulseJoint {
    let diff = from_pos - to_pos;
    let min_limit = BALL_DIAMETER;
    let max_limit = BALL_DIAMETER + BALL_RADIUS * 0.1;
    // println!(
    //     "from_pos({}, {}) to_pos({}, {}) diff({}, {}) limits({}, {})",
    //     from_pos.x, from_pos.y, to_pos.x, to_pos.y, diff.x, diff.y, min_limit, max_limit
    // );
    let prism = PrismaticJointBuilder::new(diff).limits([min_limit, max_limit]);
    ImpulseJoint::new(to_entity, prism)
}

/// build joint to each corners if entity within distance
pub fn build_corners_joints(
    commands: &mut Commands,
    grid: &Grid,
    from_entity: Entity,
    from_position: Vec2,
    to_entities: &Vec<(Entity, Vec2)>,
    connections_buffer: &mut HashMap<Entity, Vec<Entity>>,
) {
    let corners = grid.calc_corners(from_position);
    for (to_entity, to_transform) in to_entities.iter().filter(|(to_entity, to_position)| {
        *to_entity != from_entity && from_position.distance(*to_position) < BUILD_JOINT_TOLERANCE
    }) {
        let from_connections = connections_buffer.entry(from_entity).or_insert(default());
        if from_connections.contains(to_entity) {
            continue;
        } else {
            from_connections.push(*to_entity);
        }
        let to_connections = connections_buffer.entry(*to_entity).or_insert(default());
        if to_connections.contains(&from_entity) {
            continue;
        } else {
            to_connections.push(from_entity);
        }
        let mut distances = corners.map(|cor| (cor.distance(*to_transform), cor));
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));
        let closest_position = distances[0].1;

        commands.entity(from_entity).with_children(|parent| {
            parent.spawn(build_prismatic_joint(
                from_position,
                closest_position,
                *to_entity,
            ));
        });
    }
}

pub fn is_move_slow(velocity: Vec2) -> bool {
    // println!(
    //     "is_move_slow x({}) y({}) len({})",
    //     linvel.x,
    //     linvel.y,
    //     linvel.length()
    // );
    velocity.length() <= MIN_PROJECTILE_SNAP_VELOCITY
}

/// detect that projectile after snap is moving into the same clockwise/counterclockwise direction around snap grid ball
pub fn is_move_reverse(projectile_ball: &mut ProjectileBall, projectile_velocity: Vec2) -> bool {
    if projectile_ball.snap_vel == Vec2::ZERO {
        projectile_ball.snap_vel = projectile_velocity.normalize();
        // println!("snap_vel {:?}", projectile_ball.snap_vel);
    } else {
        let dot = projectile_ball
            .snap_vel
            .dot(projectile_velocity.normalize());
        // println!(
        //     "snap_vel {:?} projectile_velocity {:?} dot {}",
        //     projectile_ball.snap_vel, projectile_velocity, dot
        // );
        if dot < 0.0 {
            // println!("TRUE");
            return true;
        }
        // projectile_ball.snap_vel = projectile_velocity;
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

pub fn remove_projectile(
    commands: &mut Commands,
    projectile_entity: &Entity,
    projectile_ball: &mut ProjectileBall,
) {
    projectile_ball.is_ready_to_despawn = true;
    commands.entity(*projectile_entity).despawn_recursive();
}
