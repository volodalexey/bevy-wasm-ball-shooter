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
    panels::PanelsPlugin,
    physics::PhysicsPlugin,
    systems::{
        check_game_over, check_game_win, keydown_detect, on_begin_turn, on_snap_projectile,
        setup_gameplay,
    },
    walls::WallsPlugin,
};

mod ball;
pub mod constants;
mod events;
mod grid;
mod main_camera;
mod main_light;
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
            MainLightPlugin,
            PhysicsPlugin,
            MeshesPlugin,
            MaterialsPlugin,
            WallsPlugin,
            GridPlugin,
            ProjectilePlugin,
            PanelsPlugin,
        ))
        .add_event::<BeginTurn>()
        .insert_resource(Msaa::Off)
        .add_systems(OnEnter(AppState::Gameplay), setup_gameplay)
        .add_systems(
            Update,
            (
                on_begin_turn,
                check_game_over,
                check_game_win.after(check_game_over),
                on_snap_projectile,
                keydown_detect,
            )
                .run_if(in_state(AppState::Gameplay)),
        );
    }
}
