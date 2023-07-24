use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct SettingsButtonColors {
    pub normal: Color,
    pub normal_hovered: Color,
    pub pressed: Color,
    pub pressed_hovered: Color,
}

impl Default for SettingsButtonColors {
    fn default() -> Self {
        Self {
            normal: Color::SEA_GREEN.into(),
            normal_hovered: Color::LIME_GREEN.into(),
            pressed: Color::rgb(0.0, 0.5, 0.0).into(),
            pressed_hovered: Color::rgb(0.0, 0.4, 0.0).into(),
        }
    }
}
