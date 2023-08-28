use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::{components::AppState, ui::systems::cleanup_full_row};

use self::{
    resources::{MoveDownCounter, ScoreCounter, SpawnRowsLeft, TurnCounter},
    systems::{setup_resources, setup_ui, update_ui},
};

mod components;
pub mod resources;
mod systems;

pub struct PanelsPlugin;

impl Plugin for PanelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnCounter>()
            .init_resource::<MoveDownCounter>()
            .init_resource::<ScoreCounter>()
            .init_resource::<SpawnRowsLeft>()
            .add_systems(OnEnter(AppState::GameplayInit), (setup_resources, setup_ui))
            .add_systems(Update, update_ui.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), cleanup_full_row);
    }
}
