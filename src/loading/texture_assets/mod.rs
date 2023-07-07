mod resources;

use bevy::prelude::{App, AssetServer, Commands, Plugin, ResMut};

pub use self::resources::TextureAssets;

use super::resources::AssetsLoading;

pub struct TextureAssetsPlugin;

impl Plugin for TextureAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextureAssets>()
            .add_startup_system(load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let assets = TextureAssets {
        bevy: asset_server.load("textures/bevy.png"),
    };

    loading.0.push(assets.bevy.clone_weak_untyped());

    commands.insert_resource(assets);
}
