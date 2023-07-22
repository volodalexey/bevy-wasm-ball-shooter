use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct GameWinButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for GameWinButtonColors {
    fn default() -> Self {
        GameWinButtonColors {
            normal: Color::rgb(0.15, 1.0, 0.15).into(),
            hovered: Color::rgb(0.25, 1.0, 0.25).into(),
        }
    }
}
