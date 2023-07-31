use bevy::prelude::{App, Plugin, Vec2};

// use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

use super::grid::constants::BALL_RADIUS;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(BALL_RADIUS * 2.0),
            // RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        });
    }
}
