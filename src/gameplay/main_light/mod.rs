use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, OnExit, Plugin};

use crate::components::AppState;

use self::systems::{cleanup_main_light, setup_main_light};

pub mod components;
mod systems;

pub struct MainLightPlugin;

impl Plugin for MainLightPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Plugins
        app.add_system(setup_main_light.in_schedule(OnEnter(AppState::Gameplay)))
            .add_system(cleanup_main_light.in_schedule(OnExit(AppState::Gameplay)));
    }
}
