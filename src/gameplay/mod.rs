use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, Plugin, Update};

use crate::{components::AppState, ui::systems::interact_with_next_state_button};

use self::{
    ball::ProjectilePlugin,
    events::BeginTurn,
    grid::GridPlugin,
    main_camera::MainCameraPlugin,
    materials::MaterialsPlugin,
    meshes::MeshesPlugin,
    panels::PanelsPlugin,
    physics::PhysicsPlugin,
    systems::{check_game_over, check_game_win, keydown_detect, on_begin_turn, setup_gameplay},
    walls::WallsPlugin,
};

mod ball;
pub mod constants;
mod events;
mod grid;
mod main_camera;
mod materials;
mod meshes;
mod panels;
mod physics;
mod systems;
mod utils;
mod walls;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainCameraPlugin,
            PhysicsPlugin,
            MeshesPlugin,
            MaterialsPlugin,
            WallsPlugin,
            GridPlugin,
            ProjectilePlugin,
            PanelsPlugin,
        ))
        .add_event::<BeginTurn>()
        .add_systems(OnEnter(AppState::Gameplay), setup_gameplay)
        .add_systems(
            Update,
            (
                on_begin_turn,
                check_game_over,
                check_game_win.after(check_game_over),
                keydown_detect,
                interact_with_next_state_button,
            )
                .run_if(in_state(AppState::Gameplay)),
        );
    }
}
