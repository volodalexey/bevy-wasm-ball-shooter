use std::fmt::{Debug, Display, Formatter, Result};

use bevy::{
    prelude::{Color, Component, Vec2},
    sprite::ColorMaterial,
    time::{Timer, TimerMode},
    utils::HashSet,
};

use crate::gameplay::constants::MAX_APPEAR_TIME;

#[derive(Component)]
pub struct ProjectileBall {
    pub is_flying: bool,
    pub snap_vel: Vec2,
    pub is_snapped: bool,
}

impl Default for ProjectileBall {
    fn default() -> Self {
        Self {
            is_flying: false,
            snap_vel: Vec2::ZERO,
            is_snapped: false,
        }
    }
}

#[derive(Component)]
pub struct NextProjectileBall {}

#[derive(Component)]
pub struct GridBall {
    pub is_ready_to_despawn: bool,
}

impl Default for GridBall {
    fn default() -> Self {
        Self {
            is_ready_to_despawn: false,
        }
    }
}

#[derive(Component)]
pub struct MagneticGridBall {}

#[derive(Component)]
pub struct GridBallScaleAnimate {
    pub scale: Vec2,
    pub timer: Timer,
}

impl GridBallScaleAnimate {
    pub fn from_scale(scale: Vec2) -> Self {
        Self {
            scale,
            timer: Timer::from_seconds(fastrand::f32() * MAX_APPEAR_TIME, TimerMode::Once),
        }
    }
}

#[derive(PartialEq)]
pub enum OutBallAnimation {
    FixedCluster,
    FloatingCluster,
}

#[derive(Component)]
pub struct OutBall {
    pub started: bool,
    pub marked_for_delete: bool,
    pub animation_type: OutBallAnimation,
}

impl OutBall {
    pub fn as_fixed() -> Self {
        Self {
            started: false,
            marked_for_delete: false,
            animation_type: OutBallAnimation::FixedCluster,
        }
    }

    pub fn as_floating() -> Self {
        Self {
            started: false,
            marked_for_delete: false,
            animation_type: OutBallAnimation::FloatingCluster,
        }
    }
}

#[derive(Component, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Species {
    Red,
    Blue,
    Blau,
    Green,
    Yellow,
    White,
    Purple,
}

impl Display for Species {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Species::Red => "Species::Red",
                Species::Blue => "Species::Blue",
                Species::Blau => "Species::Blau",
                Species::Green => "Species::Green",
                Species::Yellow => "Species::Yellow",
                Species::White => "Species::White",
                Species::Purple => "Species::Purple",
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
                Species::Blau => "Species::Blau",
                Species::Green => "Species::Green",
                Species::Yellow => "Species::Yellow",
                Species::White => "Species::White",
                Species::Purple => "Species::Purple",
            },
        )
    }
}

impl Into<Color> for Species {
    fn into(self) -> Color {
        match self {
            Species::Red => Color::RED,
            Species::Blue => Color::BLUE,
            Species::Blau => Color::AQUAMARINE,
            Species::Green => Color::GREEN,
            Species::Yellow => Color::YELLOW,
            Species::White => Color::WHITE,
            Species::Purple => Color::PURPLE,
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
            1 => Species::Red,
            2 => Species::Blue,
            3 => Species::Green,
            4 => Species::Yellow,
            5 => Species::White,
            6 => Species::Blau,
            7 => Species::Purple,
            _ => unreachable!(),
        }
    }
}

impl Species {
    pub fn random_species(total_colors: u8) -> Species {
        Self::from(fastrand::u8(1..=total_colors))
    }

    pub fn pick_random(active_species: &HashSet<Species>, total_colors: u8) -> Species {
        if active_species.len() > 0 {
            let colors_in_grid: Vec<&Species> = active_species.into_iter().collect();
            let i = fastrand::usize(..colors_in_grid.len());
            *colors_in_grid[i]
        } else {
            Species::random_species(total_colors)
        }
    }
}

#[derive(Component)]
pub struct AimTarget {
    pub pointer_pressed: bool,
    pub pointer_released: bool,
    pub draw_vel: Vec2,
    pub aim_pos: Vec2,
    pub aim_vel: Vec2,
}

impl Default for AimTarget {
    fn default() -> Self {
        Self {
            pointer_pressed: false,
            pointer_released: false,
            draw_vel: Vec2::ZERO,
            aim_pos: Vec2::ZERO,
            aim_vel: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct AimLine;
