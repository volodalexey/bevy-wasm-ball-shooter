use bevy::{
    prelude::{Handle, Resource},
    text::Font,
};

#[derive(Resource, Debug)]
pub struct FontAssets {
    pub fira_sans_bold: Handle<Font>,
}

impl Default for FontAssets {
    fn default() -> Self {
        Self {
            fira_sans_bold: Handle::default(),
        }
    }
}
