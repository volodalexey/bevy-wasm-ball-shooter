use crate::{
    constants::{MAX_LEVEL, START_LEVEL},
    resources::LevelCounter,
};

pub fn increment_level(level_counter: &mut LevelCounter) {
    level_counter.0 += 1;
    if level_counter.0 > MAX_LEVEL {
        level_counter.0 = START_LEVEL
    }
}
