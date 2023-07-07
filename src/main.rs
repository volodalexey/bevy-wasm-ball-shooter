use bevy::{
    prelude::{default, App, PluginGroup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use components::AppState;
use gameplay::GameplayPlugin;
use loading::LoadingPlugin;
use start_menu::StartMenuPlugin;
use systems::exit_game;

mod components;
mod gameplay;
mod loading;
mod start_menu;
mod systems;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy+WASM Ball Shooter".into(),
                resolution: (1000., 1000.).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugin(LoadingPlugin)
        .add_plugin(StartMenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_system(exit_game)
        .run();
}
