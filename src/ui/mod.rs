use bevy::prelude::{App, Plugin, Update};

use self::{
    resources::{PointerCooldown, UIMenuButtonColors, UIMenuTextColors},
    systems::tick_pointer_cooldown_timer,
};

pub mod components;
mod constants;
pub mod resources;
pub mod systems;
pub mod utils;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointerCooldown>()
            .init_resource::<UIMenuButtonColors>()
            .init_resource::<UIMenuTextColors>()
            .add_systems(Update, tick_pointer_cooldown_timer);
    }
}
