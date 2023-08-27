use bevy::{
    prelude::{default, App, PluginGroup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use components::AppState;
use game_audio::GameAudioPlugin;
use game_over_menu::GameOverMenuPlugin;
use game_win_menu::GameWinMenuPlugin;
use gameplay::GameplayPlugin;
use loading::LoadingPlugin;
use settings_menu::SettingsMenuPlugin;
use start_menu::StartMenuPlugin;
use ui::UIPlugin;

mod components;
mod constants;
mod game_audio;
mod game_over_menu;
mod game_win_menu;
mod gameplay;
mod loading;
mod settings_menu;
mod start_menu;
mod ui;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy+WASM Ball Shooter".into(),
                    resolution: (408., 755.).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            LoadingPlugin,
            GameAudioPlugin,
            UIPlugin,
            StartMenuPlugin,
            SettingsMenuPlugin,
            GameWinMenuPlugin,
            GameplayPlugin,
            GameOverMenuPlugin,
        ))
        .add_state::<AppState>()
        .run();
}
