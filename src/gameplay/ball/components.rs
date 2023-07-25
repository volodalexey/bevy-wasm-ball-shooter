use std::fmt::{Display, Formatter, Result};

use bevy::prelude::{Color, Component, StandardMaterial};

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

#[derive(Component, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Species {
    Red,
    Blue,
    Green,
    Yellow,
    White,
}

impl Display for Species {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Species::Red => "Species::Red",
                Species::Blue => "Species::Blue",
                Species::Green => "Species::Green",
                Species::Yellow => "Species::Yellow",
                Species::White => "Species::White",
            },
        )
    }
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

    pub fn pick_random(colors_in_grid: &Vec<Species>) -> Species {
        if colors_in_grid.len() > 0 {
            let i = fastrand::usize(..colors_in_grid.len());
            colors_in_grid[i]
        } else {
            Species::random_species()
        }
    }
}

#[derive(Component)]
pub struct ProjectileArrow;
