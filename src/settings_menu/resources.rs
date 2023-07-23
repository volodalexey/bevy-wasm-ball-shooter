use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct SettingsButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for SettingsButtonColors {
    fn default() -> Self {
        Self {
            normal: Color::rgb(0.15, 1.0, 0.15).into(),
            hovered: Color::rgb(0.25, 1.0, 0.25).into(),
        }
    }
}
