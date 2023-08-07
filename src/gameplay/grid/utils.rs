use bevy::{
    prelude::{
        default, ChildBuilder, Color, Commands, DespawnRecursiveExt, Entity, Query, Vec2, With,
    },
    text::{Text, Text2dBundle, TextSection, TextStyle},
    utils::HashSet,
    window::{PrimaryWindow, Window},
};
use bevy_rapier2d::prelude::{ImpulseJoint, PrismaticJointBuilder, RevoluteJointBuilder};
use hexx::Hex;

use crate::{
    gameplay::{
        constants::{
            BALL_DIAMETER, BALL_RADIUS, MIN_PROJECTILE_SNAP_VELOCITY, PLAYGROUND_ROWS,
            PROJECTILE_SPAWN_BOTTOM, ROW_HEIGHT,
        },
        panels::resources::MoveCounter,
    },
    utils::{from_2d_to_grid_2d, from_grid_2d_to_2d},
};

use super::resources::Grid;

#[inline(always)]
pub fn find_cluster<'a, P>(grid: &Grid, origin: Hex, is_cluster: P) -> (Vec<Hex>, HashSet<Hex>)
where
    P: Fn(&Entity) -> bool,
{
    let mut processed = HashSet::<Hex>::new();
    let mut to_process = vec![origin];
    let mut cluster: Vec<Hex> = vec![];

    processed.insert(origin);

    while let Some(current) = to_process.pop() {
        if let Some(entity) = grid.get(current) {
            if !is_cluster(entity) {
                continue;
            }

            cluster.push(current);

            for (hex, _) in grid.neighbors(current).iter() {
                if processed.contains(hex) {
                    continue;
                }
                to_process.push(*hex);
                processed.insert(*hex);
            }
        }
    }

    (cluster, processed)
}

#[inline(always)]
pub fn find_floating_clusters(grid: &Grid) -> Vec<Vec<Hex>> {
    let mut processed = HashSet::<Hex>::new();
    let mut floating_clusters: Vec<Vec<Hex>> = vec![];

    for (hex, _) in grid.storage.iter() {
        if processed.contains(hex) {
            continue;
        }

        let (cluster, _processed) = find_cluster(grid, *hex, |_| true);

        processed.extend(_processed);

        if cluster.len() <= 0 {
            continue;
        }

        let mut floating = true;
        for hex in cluster.iter() {
            // TODO(pyrbin): we have to find a better way check if ball is top row
            if hex.y == 0 {
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
    let init_row_y = -(window.height()
        - PROJECTILE_SPAWN_BOTTOM
        - window.height() / 2.0
        - ROW_HEIGHT * PLAYGROUND_ROWS as f32);
    let full_height = ROW_HEIGHT * grid.init_rows as f32;
    let init_layout_y = init_row_y - full_height;
    let move_layout_y = move_counter.0 as f32 * ROW_HEIGHT;
    grid.layout.origin.x = -(match grid.init_cols & 1 == 0 {
        false => {
            (grid.init_cols as f32 / 2.0).floor() * BALL_DIAMETER + BALL_RADIUS + BALL_RADIUS / 2.0
        }
        true => (grid.init_cols as f32 / 2.0).floor() * BALL_DIAMETER + BALL_RADIUS / 2.0,
    } - BALL_RADIUS);
    grid.layout.origin.y = init_layout_y + move_layout_y;
    println!(
        "Adjust grid layout init_row_y({}) full_height({}) init_layout_y({}) grid.layout.origin({}, {})",
        init_row_y, full_height, init_layout_y, grid.layout.origin.x, grid.layout.origin.y
    );
}

pub fn clamp_inside_world_bounds(hex: &Hex, grid: &Grid) -> (Hex, bool) {
    let hex = *hex;
    let offset = hex.to_offset_coordinates(grid.offset_mode);
    let is_even = (offset[1] + 1) & 1 == 0;

    let off_q: i32 = match is_even {
        true => offset[0].clamp(
            grid.bounds.mins.init_even_off_q,
            grid.bounds.maxs.init_even_off_q,
        ),
        false => offset[0].clamp(
            grid.bounds.mins.init_odd_off_q,
            grid.bounds.maxs.init_odd_off_q,
        ),
    };
    // println!("is_even {} offset[{}, {}]", is_even, offset[0], offset[1]);

    let mut off_r = offset[1];
    if off_r < 0 {
        off_r = 0;
    }

    (
        Hex::from_offset_coordinates([off_q, off_r], grid.offset_mode),
        offset[0] != off_q || offset[1] != off_r,
    )
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

pub fn build_prismatic_joint(from_pos: Vec2, to_pos: Vec2, to_entity: &Entity) -> ImpulseJoint {
    let diff = from_pos - to_pos;
    let min_limit = BALL_DIAMETER;
    let max_limit = BALL_DIAMETER + BALL_RADIUS * 0.1;
    // println!(
    //     "from_pos({}, {}) to_pos({}, {}) diff({}, {}) limits({}, {})",
    //     from_pos.x, from_pos.y, to_pos.x, to_pos.y, diff.x, diff.y, min_limit, max_limit
    // );
    let prism = PrismaticJointBuilder::new(diff).limits([min_limit, max_limit]);
    ImpulseJoint::new(*to_entity, prism)
}

/// each ball in grid can have 4 max joints
/// assume pointy-top orientation
pub fn build_joints(hex: Hex, grid: &Grid) -> Vec<ImpulseJoint> {
    let hex_pos = from_grid_2d_to_2d(grid.layout.hex_to_world_pos(hex));

    let neighbors = vec![
        hex.neighbor(hexx::Direction::TopLeft),
        hex.neighbor(hexx::Direction::Top),
        hex.neighbor(hexx::Direction::TopRight),
        hex.neighbor(hexx::Direction::BottomRight),
    ];
    neighbors
        .iter()
        .filter_map(|neighbor_hex| {
            if let Some(grid_neighbor) = grid.get(*neighbor_hex) {
                return Some((
                    grid_neighbor,
                    from_2d_to_grid_2d(grid.layout.hex_to_world_pos(*neighbor_hex)),
                ));
            }
            None
        })
        .map(|(neighbor_entity, neighbor_pos)| {
            build_prismatic_joint(hex_pos, neighbor_pos, neighbor_entity)
        })
        .collect::<Vec<ImpulseJoint>>()
}

pub fn is_move_slow(linvel: Vec2) -> bool {
    // println!(
    //     "is_move_slow x({}) y({}) len({})",
    //     linvel.x,
    //     linvel.y,
    //     linvel.length()
    // );
    linvel.length() <= MIN_PROJECTILE_SNAP_VELOCITY || linvel.y < 0.0
}

pub fn build_ball_text(parent: &mut ChildBuilder<'_, '_, '_>, hex: Hex) {
    parent.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: format!("{}, {}", hex.x, hex.y),
                style: TextStyle {
                    color: Color::BLACK,
                    ..default()
                },
            }],
            ..default()
        },
        ..default()
    });
}

pub fn remove_projectile(commands: &mut Commands, entity: &Entity) {
    commands.entity(*entity).despawn_recursive();
}
