use bevy::prelude::{Camera, GlobalTransform, Vec2, Vec3};

/// Calculate the intersection point of a vector and a plane defined as a point and normal vector
/// where `pv` is the vector point, `dv` is the vector direction,
/// `pp` is the plane point and `np` is the planes' normal vector
pub fn plane_intersection(pv: Vec3, dv: Vec3, pp: Vec3, np: Vec3) -> Vec3 {
    let d = dv.dot(np);
    let t = (pp.dot(np) - pv.dot(np)) / d;
    pv + dv * t
}

/// Calculates origin and direction of a ray from cursor to world space.
pub fn ray_from_mouse_position(
    window_width: f32,
    window_height: f32,
    pointer_position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let x = 2.0 * (pointer_position.x / window_width) - 1.0;
    let y = 2.0 * (pointer_position.y / window_height) - 1.0;

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    return (near, dir);
}
