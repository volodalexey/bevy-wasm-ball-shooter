use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::{
    components::AppState,
    ui::systems::{cleanup_menu, interact_with_next_state_button},
};

use self::systems::{
    interact_with_level_button, interact_with_volume_button, keydown_detect, setup_menu,
};

mod components;
mod systems;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup_menu)
            .add_systems(
                Update,
                (
                    interact_with_volume_button,
                    keydown_detect,
                    interact_with_level_button,
                    interact_with_next_state_button,
                )
                    .run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), cleanup_menu);
    }
}
