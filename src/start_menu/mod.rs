use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    resources::ButtonColors,
    systems::{
        cleanup_audio, cleanup_menu, click_play_button, keydown_detect, setup_menu, start_audio,
    },
};

mod components;
mod resources;
mod systems;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_systems(OnEnter(AppState::StartMenu), (setup_menu, start_audio))
            .add_systems(
                Update,
                (click_play_button, keydown_detect).run_if(in_state(AppState::StartMenu)),
            )
            .add_systems(OnExit(AppState::StartMenu), (cleanup_menu, cleanup_audio));
    }
}
