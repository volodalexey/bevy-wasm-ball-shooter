use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct SettingsButtonColors {
    pub back_idle: Color,
    pub back_hovered: Color,
    pub back_pressed: Color,
    pub normal: Color,
    pub normal_hovered: Color,
    pub pressed: Color,
    pub pressed_hovered: Color,
}

impl Default for SettingsButtonColors {
    fn default() -> Self {
        Self {
            back_idle: Color::rgb(0.5, 0.5, 0.5),
            back_hovered: Color::rgb(0.4, 0.4, 0.4),
            back_pressed: Color::rgb(0.6, 0.6, 0.6),
            normal: Color::SEA_GREEN.into(),
            normal_hovered: Color::LIME_GREEN.into(),
            pressed: Color::rgb(0.0, 0.5, 0.0).into(),
            pressed_hovered: Color::rgb(0.0, 0.4, 0.0).into(),
        }
    }
}
