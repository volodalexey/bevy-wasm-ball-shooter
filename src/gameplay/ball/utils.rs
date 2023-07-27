use bevy::prelude::Vec2;

pub fn clamp_inside_world_bounds(mut pos: Vec2, left: f32, right: f32, top: f32) -> Vec2 {
    let (x, y) = pos.into();

    if x < left {
        pos.x = left;
    } else if x > right {
        pos.x = right;
    }

    if y < top {
        pos.y = top;
    }

    pos
}
