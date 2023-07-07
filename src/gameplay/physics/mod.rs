use bevy::prelude::{
    App, Plugin, Vec3,
};

use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

pub mod components;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::ZERO,
                ..Default::default()
            });
    }
}
