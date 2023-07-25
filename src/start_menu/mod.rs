use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::{
    components::AppState,
    ui::systems::{cleanup_menu, interact_with_next_state_button, interact_with_quit_button},
};

use self::systems::{keydown_detect, setup_menu};

mod systems;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::StartMenu), setup_menu)
            .add_systems(
                Update,
                (
                    interact_with_next_state_button,
                    keydown_detect,
                    // #[cfg(not(target_arch = "wasm32"))]
                    // #[allow(dead_code)]
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::StartMenu)),
            )
            .add_systems(OnExit(AppState::StartMenu), cleanup_menu);
    }
}
