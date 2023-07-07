use bevy::prelude::{App, Plugin};

use self::{
    audio_assets::AudioAssetsPlugin, font_assets::FontAssetsPlugin, resources::AssetsLoading,
    systems::check_assets_ready, texture_assets::TextureAssetsPlugin,
};

pub mod audio_assets;
pub mod font_assets;
mod resources;
mod systems;
pub mod texture_assets;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .add_plugin(AudioAssetsPlugin)
            .add_plugin(FontAssetsPlugin)
            .add_plugin(TextureAssetsPlugin)
            .add_system(check_assets_ready);
    }
}
