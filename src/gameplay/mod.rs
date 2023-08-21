use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, Plugin, Update};

use crate::{components::AppState, ui::systems::interact_with_next_state_button};

use self::{
    ball::ProjectilePlugin,
    events::{
        FindCluster, MoveDownLastActive, ProjectileReload, SnapProjectile, SpawnRow,
        UpdateScoreCounter,
    },
    grid::GridPlugin,
    lines::LinesPlugin,
    main_camera::MainCameraPlugin,
    materials::MaterialsPlugin,
    meshes::MeshesPlugin,
    panels::PanelsPlugin,
    physics::PhysicsPlugin,
    systems::{check_game_over, check_game_win, keydown_detect, setup_first_turn},
    walls::WallsPlugin,
};

mod ball;
pub mod constants;
mod events;
mod grid;
mod lines;
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
            LinesPlugin,
            GridPlugin,
            ProjectilePlugin,
            PanelsPlugin,
        ))
        .add_event::<ProjectileReload>()
        .add_event::<SnapProjectile>()
        .add_event::<UpdateScoreCounter>()
        .add_event::<MoveDownLastActive>()
        .add_event::<SpawnRow>()
        .add_event::<FindCluster>()
        .add_systems(OnEnter(AppState::Gameplay), setup_first_turn)
        .add_systems(
            Update,
            (keydown_detect, interact_with_next_state_button).run_if(in_state(AppState::Gameplay)),
        )
        .add_systems(
            Update,
            (check_game_over, check_game_win)
                .chain()
                .run_if(in_state(AppState::Gameplay)),
        );
    }
}
