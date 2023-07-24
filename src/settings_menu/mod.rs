use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    resources::SettingsButtonColors,
    systems::{
        cleanup_menu, interact_with_back_button, interact_with_level_button,
        interact_with_volume_button, keydown_detect, setup_menu,
    },
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
                (
                    interact_with_volume_button,
                    keydown_detect,
                    interact_with_back_button,
                    interact_with_level_button,
                )
                    .run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), cleanup_menu);
    }
}
