use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfigs, OnEnter, OnExit, OnUpdate,
    Plugin,
};

use crate::components::AppState;

use self::{
    events::BeginTurn,
    resources::{Score, TurnCounter},
    systems::{
        check_game_over, cleanup_gameplay, on_begin_turn, on_snap_projectile, setup_camera,
        setup_gameplay, setup_ui, update_ui,
    },
};

mod ball;
mod components;
mod constants;
mod events;
mod grid;
pub mod hex;
mod physics;
mod projectile;
mod resources;
mod systems;
mod utils;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BeginTurn>()
            .insert_resource(TurnCounter(0))
            .insert_resource(Score(0))
            .add_systems(
                (setup_ui, setup_camera, setup_gameplay).in_schedule(OnEnter(AppState::Gameplay)),
            )
            .add_systems(
                (
                    update_ui,
                    on_begin_turn,
                    check_game_over,
                    on_snap_projectile,
                )
                    .in_set(OnUpdate(AppState::Gameplay)),
            )
            .add_system(cleanup_gameplay.in_schedule(OnExit(AppState::Gameplay)));
    }
}
