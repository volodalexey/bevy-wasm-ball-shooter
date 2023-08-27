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
pub struct TotalColorsButton {
    pub value: u8,
    pub key: String,
    pub pressed: bool,
    pub color_type: ColorType,
}

#[derive(Component)]
pub struct TotalColumnsButton {
    pub value: u8,
    pub key: String,
    pub pressed: bool,
    pub color_type: ColorType,
}

#[derive(Component)]
pub struct TotalRowsButton {
    pub increment: bool,
    pub key: String,
    pub color_type: ColorType,
}

#[derive(Component)]
pub struct MoveDownButton {
    pub value: u8,
    pub key: String,
    pub pressed: bool,
    pub color_type: ColorType,
}
