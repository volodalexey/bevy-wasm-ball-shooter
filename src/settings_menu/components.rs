use bevy::prelude::Component;

#[derive(Component)]
pub struct SettingsMenuCamera {}

#[derive(Component)]
pub struct SettingsMenu {}

#[derive(Component, PartialEq)]
pub struct VolumeButton {
    pub value: f32,
    pub key: String,
    pub pressed: bool,
}
