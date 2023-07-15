use bevy::prelude::{App, Plugin, Update};

use self::{
    audio_assets::AudioAssetsPlugin, font_assets::FontAssetsPlugin, resources::AssetsLoading,
    systems::check_assets_ready,
};

pub mod audio_assets;
pub mod font_assets;
mod resources;
mod systems;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .add_plugins((AudioAssetsPlugin, FontAssetsPlugin))
            .add_systems(Update, check_assets_ready);
    }
}
