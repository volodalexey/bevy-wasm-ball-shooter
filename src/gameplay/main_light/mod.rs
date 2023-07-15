use bevy::prelude::{App, OnEnter, OnExit, Plugin};

use crate::components::AppState;

use self::systems::{cleanup_main_light, setup_main_light};

pub mod components;
mod systems;

pub struct MainLightPlugin;

impl Plugin for MainLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), setup_main_light)
            .add_systems(OnExit(AppState::Gameplay), cleanup_main_light);
    }
}
