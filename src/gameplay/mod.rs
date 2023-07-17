use bevy::prelude::{in_state, App, IntoSystemConfigs, Msaa, OnEnter, Plugin, Update};

use crate::components::AppState;

use self::{
    ball::ProjectilePlugin,
    events::BeginTurn,
    grid::GridPlugin,
    main_camera::MainCameraPlugin,
    main_light::MainLightPlugin,
    materials::MaterialsPlugin,
    meshes::MeshesPlugin,
    physics::PhysicsPlugin,
    resources::{RoundTurnCounter, Score, TurnCounter},
    systems::{check_game_over, on_begin_turn, on_snap_projectile, setup_gameplay},
    ui::UIPlugin,
    walls::WallsPlugin,
};

mod ball;
mod constants;
mod events;
mod grid;
mod main_camera;
mod main_light;
mod materials;
mod meshes;
mod physics;
mod resources;
mod systems;
mod ui;
mod utils;
mod walls;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainCameraPlugin,
            MainLightPlugin,
            PhysicsPlugin,
            MeshesPlugin,
            MaterialsPlugin,
            WallsPlugin,
            GridPlugin,
            ProjectilePlugin,
            UIPlugin,
        ))
        .add_event::<BeginTurn>()
        .insert_resource(Msaa::Off)
        .insert_resource(TurnCounter(0))
        .insert_resource(RoundTurnCounter(0))
        .insert_resource(Score(0))
        .add_systems(OnEnter(AppState::Gameplay), setup_gameplay)
        .add_systems(
            Update,
            (on_begin_turn, check_game_over, on_snap_projectile)
                .run_if(in_state(AppState::Gameplay)),
        );
    }
}
