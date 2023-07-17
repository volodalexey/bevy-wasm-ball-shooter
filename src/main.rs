use bevy::{
    prelude::{default, App, PluginGroup, Update},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use components::AppState;
use game_over_menu::GameOverMenuPlugin;
use gameplay::GameplayPlugin;
use loading::LoadingPlugin;
use start_menu::StartMenuPlugin;
use systems::exit_game;

mod components;
mod game_over_menu;
mod gameplay;
mod loading;
mod start_menu;
mod systems;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy+WASM Ball Shooter".into(),
                    resolution: (1000., 1000.).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            DebugLinesPlugin::default(),
            LoadingPlugin,
            StartMenuPlugin,
            GameplayPlugin,
            GameOverMenuPlugin,
        ))
        .add_state::<AppState>()
        .add_systems(Update, exit_game)
        .run();
}
