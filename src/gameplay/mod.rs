use bevy::prelude::{App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin};

use crate::components::AppState;

use self::{
    events::BeginTurn,
    grid::GridPlugin,
    main_camera::MainCameraPlugin,
    main_light::MainLightPlugin,
    physics::PhysicsPlugin,
    projectile::ProjectilePlugin,
    resources::{RoundTurnCounter, Score, TurnCounter},
    systems::{check_game_over, on_begin_turn, on_snap_projectile, setup_gameplay},
    ui::UIPlugin,
};

mod ball;
mod constants;
mod events;
mod grid;
mod main_camera;
mod main_light;
mod physics;
mod projectile;
mod resources;
mod systems;
mod ui;
mod utils;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainCameraPlugin)
            .add_plugin(MainLightPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(GridPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(UIPlugin)
            .add_event::<BeginTurn>()
            .insert_resource(TurnCounter(0))
            .insert_resource(RoundTurnCounter(0))
            .insert_resource(Score(0))
            .add_system(setup_gameplay.in_schedule(OnEnter(AppState::Gameplay)))
            .add_systems(
                (on_begin_turn, check_game_over, on_snap_projectile)
                    .in_set(OnUpdate(AppState::Gameplay)),
            );
    }
}
