use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub enum WallType {
    Left,
    Right,
    Top,
}

impl WallType {
    pub fn is_side(&self) -> bool {
        match self {
            WallType::Left | WallType::Right => true,
            WallType::Top => false,
        }
    }
    pub fn is_top(&self) -> bool {
        !self.is_side()
    }
}

#[derive(Component)]
pub struct TopWall;

#[derive(Component)]
pub struct TopWallPositionAnimate {
    pub position: Vec2,
}
