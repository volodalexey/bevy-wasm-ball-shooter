use bevy::prelude::Component;

#[derive(Component)]
pub struct SettingsMenuCamera {}

#[derive(Component)]
pub struct SettingsMenu {}

#[derive(Component)]
pub struct VolumeButton {
    pub value: f32,
    pub key: String,
    pub pressed: bool,
}

#[derive(Component)]
pub struct BackButton {}

#[derive(Component)]
pub struct LevelButton {
    pub level: u32,
    pub pressed: bool,
}
