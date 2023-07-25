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
    pub gray_hovered: Color,
    pub gray_idle: Color,
    pub gray_selected_hovered: Color,
    pub gray_selected: Color,
    pub gray_pressed: Color,

    pub green_hovered: Color,
    pub green_idle: Color,
    pub green_selected_hovered: Color,
    pub green_selected: Color,
    pub green_pressed: Color,

    pub blue_hovered: Color,
    pub blue_idle: Color,
    pub blue_selected_hovered: Color,
    pub blue_selected: Color,
    pub blue_pressed: Color,
}

impl Default for UIMenuButtonColors {
    fn default() -> Self {
        Self {
            gray_hovered: Color::rgb(0.4, 0.4, 0.4).into(),
            gray_idle: Color::rgb(0.3, 0.3, 0.3).into(),
            gray_selected_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            gray_selected: Color::rgb(0.2, 0.2, 0.2).into(),
            gray_pressed: Color::rgb(0.1, 0.1, 0.1).into(),

            green_hovered: Color::rgb(0.2, 0.8, 0.2).into(),
            green_idle: Color::rgb(0.2, 0.7, 0.2).into(),
            green_selected_hovered: Color::rgb(0.2, 0.65, 0.2).into(),
            green_selected: Color::rgb(0.2, 0.5, 0.2).into(),
            green_pressed: Color::rgb(0.2, 0.45, 0.2).into(),

            blue_hovered: Color::rgb(0.1, 0.1, 0.6).into(),
            blue_idle: Color::rgb(0.1, 0.1, 0.5).into(),
            blue_selected_hovered: Color::rgb(0.1, 0.1, 0.45).into(),
            blue_selected: Color::rgb(0.1, 0.1, 0.4).into(),
            blue_pressed: Color::rgb(0.1, 0.1, 0.3).into(),
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
