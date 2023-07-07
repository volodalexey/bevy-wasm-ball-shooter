mod resources;

use bevy::prelude::{App, AssetServer, Commands, Plugin, ResMut};

pub use self::resources::FontAssets;

use super::resources::AssetsLoading;

pub struct FontAssetsPlugin;

impl Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
            .add_startup_system(load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let assets = FontAssets {
        fira_sans_bold: asset_server.load("fonts/FiraSans-Bold.ttf"),
    };

    loading.0.push(assets.fira_sans_bold.clone_weak_untyped());

    commands.insert_resource(assets);
}
