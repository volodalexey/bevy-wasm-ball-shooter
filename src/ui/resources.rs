use bevy::{
    prelude::{Color, Resource},
    time::{Timer, TimerMode},
};

use super::constants::POINTER_COOLDOWN_TIME;

#[derive(Resource)]
pub struct PointerCooldown {
    pub timer: Timer,
    pub started: bool,
}

impl Default for PointerCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(POINTER_COOLDOWN_TIME, TimerMode::Repeating),
            started: false,
        }
    }
}

pub enum ColorType {
    Gray,
    Green,
    Blue,
}

#[derive(Resource)]
pub struct UIMenuButtonColors {
    pub gray_idle: Color,
    pub gray_hovered: Color,
    pub gray_pressed: Color,
    pub green_idle: Color,
    pub green_hovered: Color,
    pub green_pressed: Color,
    pub blue_idle: Color,
    pub blue_hovered: Color,
    pub blue_pressed: Color,
}

impl Default for UIMenuButtonColors {
    fn default() -> Self {
        Self {
            gray_idle: Color::rgb(0.15, 0.15, 0.15).into(),
            gray_hovered: Color::rgb(0.2, 0.2, 0.2).into(),
            gray_pressed: Color::rgb(0.1, 0.1, 0.1).into(),
            green_idle: Color::SEA_GREEN.into(),
            green_hovered: Color::LIME_GREEN.into(),
            green_pressed: Color::rgb(0.0, 0.5, 0.0).into(),
            blue_idle: Color::rgb(0.1, 0.1, 0.5).into(),
            blue_hovered: Color::rgb(0.1, 0.1, 0.6).into(),
            blue_pressed: Color::rgb(0.1, 0.1, 0.4).into(),
        }
    }
}

#[derive(Resource)]
pub struct UIMenuTextColors {
    pub title: Color,
    pub primary_button: Color,
}

impl Default for UIMenuTextColors {
    fn default() -> Self {
        Self {
            title: Color::rgb(0.9, 0.9, 0.9).into(),
            primary_button: Color::rgb(0.9, 0.9, 0.9).into(),
        }
    }
}
