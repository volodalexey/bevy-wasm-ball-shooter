use bevy::prelude::{Color, Component, Res, StandardMaterial};

use crate::resources::LevelCounter;

#[derive(Component)]
pub struct ProjectileBall {
    pub is_flying: bool,
}

#[derive(Component)]
pub struct ProjectileLine;

#[derive(Component)]
pub struct ProjectileLineParent;

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
    pub fn random_species(level_counter: &Res<LevelCounter>) -> Species {
        let range = match level_counter.0 {
            1 => 1, // one color
            2..=3 => fastrand::u8(0..2),
            4..=5 => fastrand::u8(0..3),
            6..=7 => fastrand::u8(0..4),
            _ => fastrand::u8(0..5),
        };
        match range {
            0 => Species::Red,
            1 => Species::Blue,
            2 => Species::Green,
            3 => Species::Yellow,
            4 => Species::White,
            _ => unreachable!(),
        }
    }
}

#[derive(Component)]
pub struct ProjectileArrow;
