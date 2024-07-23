use bevy::prelude::*;
// use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands, player: Query<&Transform, With<Player>>) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn camera_follow(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };

    for mut transform in &mut camera {
        let pos = player_transform.translation;
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
