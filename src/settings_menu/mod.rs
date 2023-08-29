use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::{
    components::AppState,
    ui::systems::{cleanup_menu, interact_with_next_state_button},
};

use self::systems::{
    colors_systems::interact_with_colors_button,
    columns_systems::interact_with_columns_button,
    keydown_systems::keydown_detect,
    menu_systems::setup_menu,
    move_down_systems::interact_with_move_down_button,
    rows_systems::{
        interact_with_init_rows_button, interact_with_total_rows_button, update_rows_text,
    },
    volume_systems::interact_with_volume_button,
};

mod components;
mod systems;
pub mod utils;
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup_menu)
            .add_systems(
                Update,
                (
                    interact_with_volume_button,
                    keydown_detect,
                    interact_with_next_state_button,
                    interact_with_colors_button,
                    interact_with_columns_button,
                    interact_with_init_rows_button,
                    interact_with_total_rows_button,
                    interact_with_move_down_button,
                    update_rows_text,
                )
                    .run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), cleanup_menu);
    }
}
