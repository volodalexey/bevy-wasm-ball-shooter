use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::components::AppState;

use self::systems::{cleanup_main_camera, control_camera_position, setup_main_camera};

pub mod components;
mod systems;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Plugins
        app.add_system(setup_main_camera.in_schedule(OnEnter(AppState::Gameplay)))
            .add_system(control_camera_position.in_set(OnUpdate(AppState::Gameplay)))
            .add_system(cleanup_main_camera.in_schedule(OnExit(AppState::Gameplay)));
    }
}
