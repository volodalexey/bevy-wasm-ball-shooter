use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
use crate::ui::systems::interact_with_quit_button;
use crate::{
    components::AppState,
    ui::systems::{cleanup_menu, interact_with_next_state_button},
};

use self::systems::{keydown_detect, setup_menu};

mod systems;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_menu)
            .add_systems(
                Update,
                (
                    interact_with_next_state_button,
                    keydown_detect,
                    #[cfg(not(target_arch = "wasm32"))]
                    #[allow(dead_code)]
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), cleanup_menu);
    }
}
