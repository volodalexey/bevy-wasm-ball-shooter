mod resources;

use bevy::prelude::{App, AssetServer, Commands, Plugin, ResMut, Startup};

pub use self::resources::SpriteAssets;

use super::resources::AssetsLoading;

pub struct SpriteAssetsPlugin;

impl Plugin for SpriteAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteAssets>()
            .add_systems(Startup, load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let assets = SpriteAssets {
        arrow_up_right_from_square: asset_server.load("sprites/arrow-up-right-from-square.png"),
    };

    loading
        .0
        .push(assets.arrow_up_right_from_square.clone_weak_untyped());

    commands.insert_resource(assets);
}
