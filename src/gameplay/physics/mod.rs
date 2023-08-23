use bevy::prelude::{App, Plugin, Vec2};
use bevy_xpbd_2d::{prelude::PhysicsPlugins, resources::Gravity};

pub mod layers;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vec2::ZERO));
    }
}
