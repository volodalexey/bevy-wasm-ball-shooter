use bevy::prelude::Component;
use hexx::Hex;

#[derive(Component)]
pub struct HexComponent {
    pub hex: Hex,
}
