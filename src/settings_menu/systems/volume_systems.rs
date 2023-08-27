use bevy::{
    prelude::{AudioSink, Commands, Query, Res, ResMut, With},
    ui::{BackgroundColor, Interaction},
};
use bevy_pkv::PkvStore;

use crate::{
    game_audio::{
        components::MainSound,
        constants::{MAIN_SOUND_VOLUME_KEY, SFX_SOUND_VOLUME_KEY},
        utils::{play_score_audio, play_shoot_audio, toggle_main_audio},
    },
    loading::audio_assets::AudioAssets,
    settings_menu::components::VolumeButton,
    ui::{resources::UIMenuButtonColors, utils::button_utils::button_color_by_interaction},
};

pub fn interact_with_volume_button(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    main_sound_query: Query<&AudioSink, With<MainSound>>,
    button_colors: Res<UIMenuButtonColors>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut VolumeButton),
        With<VolumeButton>,
    >,
    mut pkv: ResMut<PkvStore>,
) {
    let pressed_button: (i32, String) = match button_query
        .iter()
        .enumerate()
        .find(|(_, (interaction, _, _))| **interaction == Interaction::Pressed)
    {
        Some((idx, (_, _, button_volume))) => (idx as i32, button_volume.key.clone()),
        None => (-1, "".to_string()),
    };
    for (idx, (interaction, mut background_color, mut button_volume)) in
        button_query.iter_mut().enumerate()
    {
        match *interaction {
            Interaction::Pressed => {
                if !button_volume.pressed {
                    button_volume.pressed = true;
                    pkv.set_string(button_volume.key.clone(), &button_volume.value.to_string())
                        .expect("failed to save volume");
                    *background_color = button_color_by_interaction(
                        button_volume.pressed,
                        &button_colors,
                        &button_volume.color_type,
                        interaction,
                    )
                    .into();

                    match button_volume.key.as_str() {
                        MAIN_SOUND_VOLUME_KEY => {
                            toggle_main_audio(&main_sound_query, button_volume.value);
                        }
                        SFX_SOUND_VOLUME_KEY => {
                            if fastrand::bool() {
                                play_shoot_audio(&mut commands, &audio_assets, button_volume.value);
                            } else {
                                play_score_audio(&mut commands, &audio_assets, button_volume.value);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = button_color_by_interaction(
                    button_volume.pressed,
                    &button_colors,
                    &button_volume.color_type,
                    interaction,
                )
                .into();
            }
            Interaction::None => {
                if pressed_button.0 > -1
                    && pressed_button.0 != idx as i32
                    && pressed_button.1 == button_volume.key
                {
                    if button_volume.pressed {
                        button_volume.pressed = false;
                    }
                }
                *background_color = button_color_by_interaction(
                    button_volume.pressed,
                    &button_colors,
                    &button_volume.color_type,
                    interaction,
                )
                .into();
            }
        };
    }
}
