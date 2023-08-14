use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TurnCounter(pub u32);

#[derive(Resource)]
pub struct MoveCounter(pub u32);

#[derive(Resource)]
pub struct CooldownMoveCounter {
    pub value: u32,
    pub init_value: u32,
}

impl Default for CooldownMoveCounter {
    fn default() -> Self {
        Self {
            value: 0,
            init_value: 0,
        }
    }
}

impl CooldownMoveCounter {
    pub fn from_level(level_counter: u32) -> Self {
        let init_value = match level_counter {
            1 => 1,
            2 => 1,
            3 => 1,
            4 => 8,
            5 => 7,
            6 => 6,
            _ => 5,
        };
        Self {
            value: init_value,
            init_value,
        }
    }
}

#[derive(Resource)]
pub struct ScoreCounter(pub u32);
