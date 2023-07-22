use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct StartMenuButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for StartMenuButtonColors {
    fn default() -> Self {
        StartMenuButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}
