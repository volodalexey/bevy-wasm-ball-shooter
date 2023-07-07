use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    StartMenu,
    Gameplay,
    GameOver,
}
