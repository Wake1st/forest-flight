use crate::{player::Player, state::GameState};
use bevy::{
    core_pipeline::{bloom::BloomSettings, experimental::taa::TemporalAntiAliasBundle},
    pbr::ScreenSpaceAmbientOcclusionBundle,
    prelude::*,
};

const CAM_DISTANCE: f32 = 25.0;
const CAM_HEIGHT: f32 = 16.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::Ready)),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                    ..default()
                },
                projection: bevy::prelude::Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::PI / 6.0,
                    ..default()
                }),
                ..default()
            },
            BloomSettings {
                intensity: 0.3,
                ..default()
            },
        ))
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(TemporalAntiAliasBundle::default());
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mut transform = query.single_mut();

    for &player_transform in &player_query {
        let p_trans = player_transform.translation;
        let p_euler = player_transform.rotation.to_euler(EulerRot::YXZ);

        transform.translation.y = p_trans.y + CAM_HEIGHT;
        transform.translation.x = p_trans.x + CAM_DISTANCE * f32::sin(p_euler.0);
        transform.translation.z = p_trans.z + CAM_DISTANCE * f32::cos(p_euler.0);
        transform.look_at(Vec3::new(p_trans.x, 0.0, p_trans.z), Vec3::Y);
    }
}
