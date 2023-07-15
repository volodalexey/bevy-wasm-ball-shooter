use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    resources::ButtonColors,
    systems::{cleanup_menu, click_play_button, setup_menu},
};

mod components;
mod resources;
mod systems;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_systems(OnEnter(AppState::GameOver), setup_menu)
            .add_systems(
                Update,
                click_play_button.run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), cleanup_menu);
    }
}
