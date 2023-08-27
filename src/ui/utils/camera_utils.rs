use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{default, Camera2d, Camera2dBundle, Color, Commands},
};

use crate::ui::components::UICamera;

pub fn build_ui_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        UICamera {},
    ));
}
