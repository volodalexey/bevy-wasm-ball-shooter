use bevy::app::AppExit;
use bevy::prelude::{Commands, EventWriter, Input, KeyCode, Res, ResMut};
use bevy::time::Time;
use bevy_pkv::PkvStore;

use crate::gameplay::constants::{DEFAULT_LEVEL, LEVEL_KEY};
use crate::resources::{LevelCounter, PointerCooldown};

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn tick_pointer_cooldown_timer(mut pointer_cooldown: ResMut<PointerCooldown>, time: Res<Time>) {
    if pointer_cooldown.started {
        pointer_cooldown.timer.tick(time.delta());
        if pointer_cooldown.timer.finished() {
            pointer_cooldown.started = false;
        }
    }
}

pub fn load_saved_level(mut commands: Commands, pkv: Res<PkvStore>) {
    if let Ok(level) = pkv.get::<String>(LEVEL_KEY) {
        if let Ok(level) = level.parse::<u32>() {
            commands.insert_resource(LevelCounter(level));
            return;
        }
    }
    commands.insert_resource(LevelCounter(DEFAULT_LEVEL));
}
