use bevy::prelude::Component;

#[derive(Component)]
pub enum LineType {
    GridTop,
    GridBottom,
    GameOver,
}
