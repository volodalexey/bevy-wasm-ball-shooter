use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfig, OnEnter, OnExit, OnUpdate,
    Plugin,
};

use crate::components::AppState;

use self::{
    resources::ButtonColors,
    systems::{cleanup_menu, click_play_button, setup_menu, start_audio},
};

mod resources;
mod systems;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_systems((setup_menu, start_audio).in_schedule(OnEnter(AppState::StartMenu)))
            .add_system(click_play_button.in_set(OnUpdate(AppState::StartMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::StartMenu)));
    }
}
