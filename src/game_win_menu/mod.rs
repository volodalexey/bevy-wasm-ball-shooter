use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::components::AppState;

use self::{
    resources::GameWinButtonColors,
    systems::{cleanup_menu, click_play_button, keydown_detect, setup_menu},
};

mod components;
mod resources;
mod systems;

pub struct GameWinMenuPlugin;

impl Plugin for GameWinMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWinButtonColors>()
            .add_systems(OnEnter(AppState::GameWin), setup_menu)
            .add_systems(
                Update,
                (click_play_button, keydown_detect).run_if(in_state(AppState::GameWin)),
            )
            .add_systems(OnExit(AppState::GameWin), cleanup_menu);
    }
}
