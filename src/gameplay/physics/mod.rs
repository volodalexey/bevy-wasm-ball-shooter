use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update, Vec2},
};
use bevy_xpbd_2d::{prelude::PhysicsPlugins, resources::Gravity};

use crate::components::AppState;

use self::systems::{cleanup_fps_text, control_physics, setup_fps_text, update_fps_text};

pub mod components;
pub mod layers;
pub mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(), FrameTimeDiagnosticsPlugin))
            .add_systems(OnEnter(AppState::Gameplay), setup_fps_text)
            .add_systems(
                Update,
                (control_physics, update_fps_text).run_if(in_state(AppState::Gameplay)),
            )
            .add_systems(OnExit(AppState::Gameplay), cleanup_fps_text)
            .insert_resource(Gravity(Vec2::ZERO));
    }
}
