use std::fmt::{Debug, Display, Formatter, Result};

use bevy::{
    prelude::{Color, Component},
    sprite::ColorMaterial,
};
use hexx::Hex;

#[derive(Component)]
pub struct ProjectileBall {
    pub is_flying: bool,
    pub is_ready_to_despawn: bool,
}

#[derive(Component)]
pub struct GridBall {
    pub hex: Hex,
}

#[derive(Component)]
pub struct OutBall {
    pub started: bool,
    pub marked_for_delete: bool,
}

impl Default for OutBall {
    fn default() -> Self {
        Self {
            started: false,
            marked_for_delete: false,
        }
    }
}

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

impl Debug for Species {
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

impl Into<ColorMaterial> for Species {
    fn into(self) -> ColorMaterial {
        let color: Color = self.into();
        color.into()
    }
}

impl From<u8> for Species {
    fn from(num: u8) -> Self {
        match num {
            0 => Species::Red,
            1 => Species::Blue,
            2 => Species::Green,
            3 => Species::Yellow,
            4 => Species::White,
            _ => unreachable!(),
        }
    }
}

impl Species {
    pub fn random_species() -> Species {
        Self::from(fastrand::u8(0..5))
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
pub struct AimLine;

#[derive(Component)]
pub struct AimTarget;
