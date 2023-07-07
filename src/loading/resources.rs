use bevy::prelude::{HandleUntyped, Resource};

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);
