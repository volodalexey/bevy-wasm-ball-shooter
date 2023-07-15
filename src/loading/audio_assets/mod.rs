mod resources;
mod systems;

use bevy::prelude::{App, Plugin, Startup};

pub use self::resources::AudioAssets;
use self::systems::load_assets;

pub struct AudioAssetsPlugin;

impl Plugin for AudioAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_systems(Startup, load_assets);
    }
}
