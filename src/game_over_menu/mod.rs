use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::{
    components::AppState,
    ui::systems::{cleanup_menu, interact_with_next_state_button, interact_with_quit_button},
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
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), cleanup_menu);
    }
}
