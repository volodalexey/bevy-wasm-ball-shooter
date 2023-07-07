use bevy::prelude::{Audio, EventReader, Res};

use super::events::{AudioEvent, AudioLoopEvent};

pub fn on_audio_event(audio: Res<Audio>, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }
    for event in audio_events.iter() {
        audio.play(event.clip.clone_weak());
    }
}

pub fn on_audio_loop_event(audio: Res<Audio>, mut audio_events: EventReader<AudioLoopEvent>) {
    if audio_events.is_empty() {
        return;
    }
    for event in audio_events.iter() {
        audio.play(event.clip.clone_weak());
    }
}
