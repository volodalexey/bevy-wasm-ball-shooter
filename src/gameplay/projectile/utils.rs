use bevy::prelude::Vec3;

use crate::gameplay::grid::resources::Bounds;

pub fn clamp_inside_world_bounds(
    mut pos: Vec3,
    size: f32,
    grid_bounds: &Bounds,
) -> (Vec3, bool, bool) {
    let (x, _, y) = pos.into();

    let mut clamped_x = false;
    let mut clamped_y = false;

    let (x0, x1) = (x - size, x + size);
    let y1 = y - size;

    if x0 <= grid_bounds.mins.x {
        pos.x = grid_bounds.mins.x + size;
        clamped_x = true;
    } else if x1 >= grid_bounds.maxs.x {
        pos.x = grid_bounds.maxs.x - size;
        clamped_x = true;
    }

    if y1 <= grid_bounds.mins.y {
        pos.y = grid_bounds.mins.y - size;
        clamped_y = true;
    }

    (pos, clamped_x, clamped_y)
}
