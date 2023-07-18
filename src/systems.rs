use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Input, KeyCode, Res, ResMut};
use bevy::time::Time;

use crate::resources::PointerCooldown;

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
