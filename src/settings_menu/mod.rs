use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    resources::SettingsButtonColors,
    systems::{cleanup_menu, interact_with_, keydown_detect, setup_menu},
};

mod components;
mod resources;
mod systems;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SettingsButtonColors>()
            .add_systems(OnEnter(AppState::Settings), setup_menu)
            .add_systems(
                Update,
                (interact_with_, keydown_detect).run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), cleanup_menu);
    }
}
