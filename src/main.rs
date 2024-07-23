mod camera;
mod light;
mod player;

use bevy::prelude::*;
use bevy_sprite3d::Sprite3dPlugin;

use camera::CameraPlugin;
use light::LightPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Forest Flight".into(),
                        position: WindowPosition::At(IVec2 { x: 50, y: 70 }),
                        resolution: (1800.0, 900.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins((Sprite3dPlugin, PlayerPlugin, CameraPlugin, LightPlugin))
        .run();
}
