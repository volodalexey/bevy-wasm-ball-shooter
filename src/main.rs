use bevy::{
    prelude::{default, App, PluginGroup, Update},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use components::AppState;
use game_audio::GameAudioPlugin;
use game_over_menu::GameOverMenuPlugin;
use game_win_menu::GameWinMenuPlugin;
use gameplay::GameplayPlugin;
use loading::LoadingPlugin;
use resources::{LevelCounter, PointerCooldown};
use settings_menu::SettingsMenuPlugin;
use start_menu::StartMenuPlugin;
use systems::{exit_game, tick_pointer_cooldown_timer};

mod components;
mod game_audio;
mod game_over_menu;
mod game_win_menu;
mod gameplay;
mod loading;
mod resources;
mod settings_menu;
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
            LoadingPlugin,
            GameAudioPlugin,
            StartMenuPlugin,
            SettingsMenuPlugin,
            GameWinMenuPlugin,
            GameplayPlugin,
            GameOverMenuPlugin,
        ))
        .add_state::<AppState>()
        .insert_resource(LevelCounter(1))
        .init_resource::<PointerCooldown>()
        .add_systems(Update, (exit_game, tick_pointer_cooldown_timer))
        .run();
}
