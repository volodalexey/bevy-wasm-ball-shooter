use bevy::{
    asset::{HandleId, LoadState},
    prelude::{error, AssetServer, Commands, NextState, Res, ResMut},
};

use crate::components::AppState;

use super::resources::AssetsLoading;

pub fn check_group_load_state(
    asset_server: &Res<AssetServer>,
    handles: impl IntoIterator<Item = HandleId>,
) -> (Option<HandleId>, LoadState) {
    let mut load_state = (None, LoadState::Loaded);
    for handle_id in handles {
        match handle_id {
            HandleId::AssetPathId(id) => match asset_server.get_load_state(id) {
                LoadState::Loaded => continue,
                LoadState::Loading => {
                    load_state = (Some(handle_id), LoadState::Loading);
                }
                LoadState::Failed => {
                    return (Some(handle_id), LoadState::Failed);
                }
                LoadState::NotLoaded => {
                    return (Some(handle_id), LoadState::NotLoaded);
                }
                LoadState::Unloaded => {
                    return (Some(handle_id), LoadState::Unloaded);
                }
            },
            HandleId::Id(_, _) => {
                return (Some(handle_id), LoadState::NotLoaded);
            }
        }
    }

    load_state
}

pub fn check_assets_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    some_assets_loading: Option<Res<AssetsLoading>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Some(assets_loading) = some_assets_loading {
        let (some_handle_id, load_state) =
            check_group_load_state(&asset_server, assets_loading.0.iter().map(|h| h.id()));
        match load_state {
            LoadState::Failed => match some_handle_id {
                Some(handle_id) => match asset_server.get_handle_path(handle_id) {
                    Some(handle_path) => error!("Asset {:?} failed to load!", handle_path),
                    None => error!("Unable to get handle path"),
                },
                None => error!("Unable to extract handle id"),
            },
            LoadState::Loaded => {
                commands.remove_resource::<AssetsLoading>();
                app_state_next_state.set(AppState::StartMenu);
            }
            _ => {
                // NotLoaded/Loading: not fully ready yet
            }
        }
    }
}
