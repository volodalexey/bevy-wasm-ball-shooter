use bevy::prelude::{App, OnEnter, Plugin};
use bevy_pkv::PkvStore;

use crate::components::AppState;

use self::systems::check_start_main_audio;

mod components;
pub mod constants;
mod systems;
mod utils;
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PkvStore::new("bevy-wasm", "ball-shooter"))
            .add_systems(OnEnter(AppState::StartMenu), check_start_main_audio);
    }
}
