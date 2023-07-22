use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct GameOverButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for GameOverButtonColors {
    fn default() -> Self {
        GameOverButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}
