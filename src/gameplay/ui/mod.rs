use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::components::AppState;

use self::systems::{cleanup_ui, setup_ui, update_ui};

mod components;
mod systems;
mod utils;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnEnter(AppState::Gameplay)))
            .add_system(update_ui.in_set(OnUpdate(AppState::Gameplay)))
            .add_system(cleanup_ui.in_schedule(OnExit(AppState::Gameplay)));
    }
}
