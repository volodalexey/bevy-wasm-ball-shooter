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
            1 => 0,
            2 => 10,
            3 => 9,
            4 => 8,
            5 => 7,
            6 => 6,
            7 => 5,
            8 => 4,
            9 => 3,
            10 => 2,
            _ => 1,
        };
        Self {
            value: init_value,
            init_value,
        }
    }
}

#[derive(Resource)]
pub struct ScoreCounter(pub u32);
