use bevy::prelude::{App, OnEnter, OnExit, Plugin};

use crate::components::AppState;

use self::systems::{cleanup_main_camera, setup_main_camera};

pub mod components;
mod systems;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), setup_main_camera)
            // .add_systems(
            //     Update,
            //     control_camera_position.run_if(in_state(AppState::Gameplay)),
            // )
            .add_systems(OnExit(AppState::Gameplay), cleanup_main_camera);
    }
}
