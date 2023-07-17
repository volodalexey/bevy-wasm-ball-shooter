use bevy::prelude::{Color, Component, StandardMaterial};

#[derive(Component)]
pub struct ProjectileBall {
    pub is_flying: bool,
}

#[derive(Component)]
pub struct GridBall;

#[derive(Component, PartialEq, Clone, Copy)]
pub enum Species {
    Red,
    Blue,
    Green,
    Yellow,
    White,
}

impl Into<Color> for Species {
    fn into(self) -> Color {
        match self {
            Species::Red => Color::rgb_u8(244, 47, 47),
            Species::Blue => Color::rgb_u8(0, 93, 234),
            Species::Green => Color::rgb_u8(0, 197, 171),
            Species::Yellow => Color::rgb_u8(255, 219, 0),
            Species::White => Color::ANTIQUE_WHITE,
        }
    }
}

impl Into<StandardMaterial> for Species {
    fn into(self) -> StandardMaterial {
        let color: Color = self.into();
        color.into()
    }
}

impl Species {
    pub fn random_species() -> Species {
        match fastrand::u8(0..5) {
            0 => Species::Red,
            1 => Species::Blue,
            2 => Species::Green,
            3 => Species::Yellow,
            4 => Species::White,
            _ => unreachable!(),
        }
    }
}
