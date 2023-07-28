use bevy::{prelude::Entity, utils::HashSet};
use hexx::Hex;

use crate::gameplay::panels::resources::MoveCounter;

use super::{resources::Grid, systems::VISIBLE_ROWS};

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

pub fn adjust_grid_layout(grid: &mut Grid, move_counter: &MoveCounter) {
    let row_height = 1.5 * grid.layout.hex_size.y;
    let init_layout_y = -grid.init_rows as f32 * row_height + VISIBLE_ROWS * row_height;
    let move_layout_y = move_counter.0 as f32 * row_height;
    grid.layout.origin.y = init_layout_y + move_layout_y;
}

pub fn clamp_inside_world_bounds(hex: &Hex, grid: &Grid) -> Hex {
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

    Hex::from_offset_coordinates([off_q, off_r], grid.offset_mode)
}
