use bevy::prelude::Component;

use crate::components::AppState;

use super::resources::ColorType;

#[derive(Component)]
pub struct UICamera {}

#[derive(Component)]
pub struct UIMenu {}

#[derive(Component)]
pub struct QuitButton {
    pub color_type: ColorType,
}

#[derive(Component)]
pub struct NextStateButton {
    pub color_type: ColorType,
    pub next_state: AppState,
}
