use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

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
            .add_system(setup_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(click_play_button.in_set(OnUpdate(AppState::GameOver)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
