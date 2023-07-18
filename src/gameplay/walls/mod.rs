use bevy::prelude::{
    any_with_component, in_state, not, App, Condition, IntoSystemConfigs, OnExit, Plugin, Update,
};

use crate::AppState;

use self::{
    components::WallType,
    systems::{cleanup_level_walls, setup_level_walls},
};

pub mod components;
mod systems;
mod wall_bundle;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_level_walls.run_if(
                in_state(AppState::Gameplay).and_then(not(any_with_component::<WallType>())),
            ),
        )
        .add_systems(OnExit(AppState::Gameplay), cleanup_level_walls);
    }
}
