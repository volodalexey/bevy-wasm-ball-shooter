use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    resources::StartMenuButtonColors,
    systems::{
        cleanup_menu, interact_with_play_button, interact_with_settings_button, keydown_detect,
        setup_menu,
    },
};

mod components;
mod resources;
mod systems;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StartMenuButtonColors>()
            .add_systems(OnEnter(AppState::StartMenu), setup_menu)
            .add_systems(
                Update,
                (
                    interact_with_play_button,
                    interact_with_settings_button,
                    keydown_detect,
                )
                    .run_if(in_state(AppState::StartMenu)),
            )
            .add_systems(OnExit(AppState::StartMenu), cleanup_menu);
    }
}
