use bevy::{prelude::Entity, utils::HashSet};

use crate::gameplay::hex::Coord;

use super::resources::Grid;

#[inline(always)]
pub fn find_cluster<'a, P>(
    grid: &Grid,
    origin: Coord,
    is_cluster: P,
) -> (Vec<Coord>, HashSet<Coord>)
where
    P: Fn(&Entity) -> bool,
{
    let mut processed = HashSet::<Coord>::new();
    let mut to_process = vec![origin];
    let mut cluster: Vec<Coord> = vec![];

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
pub fn find_floating_clusters(grid: &Grid) -> Vec<Vec<Coord>> {
    let mut processed = HashSet::<Coord>::new();
    let mut floating_clusters: Vec<Vec<Coord>> = vec![];

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
            if hex.r == 0 {
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
