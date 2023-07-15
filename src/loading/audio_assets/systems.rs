use bevy::prelude::{AssetServer, Commands, ResMut};

use crate::loading::resources::AssetsLoading;

use super::AudioAssets;

pub fn load_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let assets = AudioAssets {
        flying: asset_server.load("audio/flying.ogg"),
        soundtrack: asset_server.load("audio/soundtrack.ogg"),
        score: asset_server.load("audio/score.ogg"),
    };

    loading.0.push(assets.flying.clone_weak_untyped());
    loading.0.push(assets.soundtrack.clone_weak_untyped());
    loading.0.push(assets.score.clone_weak_untyped());

    commands.insert_resource(assets);
}
