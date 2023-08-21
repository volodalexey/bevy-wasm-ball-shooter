use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::{components::AppState, ui::systems::cleanup_full_row};

use self::{
    resources::{CooldownMoveCounter, MoveCounter, ScoreCounter, TurnCounter},
    systems::{setup_resources, setup_ui, update_ui},
};

mod components;
pub mod resources;
mod systems;

pub struct PanelsPlugin;

impl Plugin for PanelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnCounter(0))
            .insert_resource(MoveCounter(0))
            .insert_resource(CooldownMoveCounter::default())
            .insert_resource(ScoreCounter(0))
            .add_systems(OnEnter(AppState::GameplayInit), (setup_resources, setup_ui))
            .add_systems(Update, update_ui.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), cleanup_full_row);
    }
}
