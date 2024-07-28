use bevy::prelude::*;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    SpawnPlayer,
    Ready,
}
