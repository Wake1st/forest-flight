mod assets;
mod camera;
mod light;
mod player;
mod state;

use bevy::{prelude::*, window::WindowResolution};
use bevy_sprite3d::Sprite3dPlugin;

use assets::AssetsPlugin;
use camera::CameraPlugin;
use light::LightPlugin;
use player::PlayerPlugin;
use state::GameState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1080.0, 1080.0 * 3. / 4.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .add_plugins((
            Sprite3dPlugin,
            PlayerPlugin,
            CameraPlugin,
            AssetsPlugin,
            LightPlugin,
        ))
        .run();
}
