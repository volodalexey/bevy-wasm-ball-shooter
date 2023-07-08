use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, OnExit, Plugin};

use crate::components::AppState;

use self::systems::{cleanup_main_camera, setup_main_camera};

pub mod components;
mod systems;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Plugins
        app.add_system(setup_main_camera.in_schedule(OnEnter(AppState::Gameplay)))
            .add_system(cleanup_main_camera.in_schedule(OnExit(AppState::Gameplay)));
    }
}
