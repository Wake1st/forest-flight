use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::state::GameState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, check_assets.run_if(in_state(GameState::Loading)))
            .insert_resource(ImageAssets::default());
    }
}

#[derive(Resource, Default)]
pub struct ImageAssets {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

fn setup(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<ImageAssets>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    assets.image = asset_server.load("atlas.png");

    assets.layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        2,
        4,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    ));
}

fn check_assets(
    asset_server: Res<AssetServer>,
    assets: Res<ImageAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.get_load_state(assets.image.id()) == Some(LoadState::Loaded) {
        next_state.set(GameState::SpawnPlayer);
    }
}
