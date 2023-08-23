use bevy_xpbd_2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum Layer {
    Projectile,
    Walls,
    Grid,
    Out,
    Lines,
}
