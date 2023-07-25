use bevy::prelude::Component;

use crate::ui::resources::ColorType;

#[derive(Component)]
pub struct VolumeButton {
    pub value: f32,
    pub key: String,
    pub pressed: bool,
    pub color_type: ColorType,
}

#[derive(Component)]
pub struct LevelButton {
    pub level: u32,
    pub pressed: bool,
    pub color_type: ColorType,
}
