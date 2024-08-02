use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::state::GameState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_small_assets, setup_large_assets))
            .add_systems(
                Update,
                (check_small_assets, check_large_assets).run_if(in_state(GameState::Loading)),
            )
            .insert_resource(SmallImageAssets::default())
            .insert_resource(LargeImageAssets::default());
    }
}

#[derive(Resource, Default)]
pub struct SmallImageAssets {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default)]
pub struct LargeImageAssets {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

fn setup_small_assets(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<SmallImageAssets>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    assets.image = asset_server.load("player.png");

    assets.layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        1,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    ));
}

fn setup_large_assets(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<LargeImageAssets>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    assets.image = asset_server.load("tree.png");

    assets.layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 512),
        1,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(192, 0)),
    ));
}

fn check_small_assets(
    asset_server: Res<AssetServer>,
    assets: Res<SmallImageAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.get_load_state(assets.image.id()) == Some(LoadState::Loaded) {
        next_state.set(GameState::SpawnPlayer);
    }
}

fn check_large_assets(
    asset_server: Res<AssetServer>,
    assets: Res<LargeImageAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.get_load_state(assets.image.id()) == Some(LoadState::Loaded) {
        next_state.set(GameState::SpawnPlayer);
    }
}
