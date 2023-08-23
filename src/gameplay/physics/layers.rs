use bevy_xpbd_2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum Layer {
    Walls,
    Grid,
    Out,
    Lines,
}
