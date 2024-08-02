use crate::{player::Player, state::GameState};
use bevy::{
    asset::LoadState,
    core_pipeline::{bloom::BloomSettings, experimental::taa::TemporalAntiAliasBundle, Skybox},
    math::vec3,
    pbr::ScreenSpaceAmbientOcclusionBundle,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        texture::CompressedImageFormats,
    },
};

const CAM_DISTANCE: f32 = 25.0;
const CAM_HEIGHT: f32 = 6.0;

const CUBEMAP: (&str, CompressedImageFormats) = ("cubemap.png", CompressedImageFormats::NONE);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn_camera)
            .add_systems(Update, asset_loaded.run_if(in_state(GameState::Ready)));
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

fn spawn_camera(
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let (player_entity, player_transform) = player.single();
    let mut camera_transform = Transform::from_translation(
        player_transform.translation + vec3(0.0, CAM_HEIGHT, CAM_DISTANCE),
    );
    camera_transform.look_at(player_transform.translation, Vec3::Y);

    let skybox_handle = asset_server.load(CUBEMAP.0);

    commands.entity(player_entity).with_children(|parent| {
        parent
            .spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        ..default()
                    },
                    projection: bevy::prelude::Projection::Perspective(PerspectiveProjection {
                        fov: std::f32::consts::PI / 6.0,
                        ..default()
                    }),
                    transform: camera_transform,
                    ..default()
                },
                Skybox {
                    image: skybox_handle.clone(),
                    brightness: 1000.0,
                    // ..default()
                },
                BloomSettings {
                    intensity: 0.3,
                    ..default()
                },
            ))
            .insert(ScreenSpaceAmbientOcclusionBundle::default())
            .insert(TemporalAntiAliasBundle::default());
    });

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

// fn camera_follow_player(
//     player_query: Query<&Transform, With<Player>>,
//     mut query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
// ) {
//     let mut transform = query.single_mut();

//     for &player_transform in &player_query {
//         let p_trans = player_transform.translation;
//         let p_euler = player_transform.rotation.to_euler(EulerRot::YXZ);

//         transform.translation.y = p_trans.y + CAM_HEIGHT;
//         transform.translation.x = p_trans.x + CAM_DISTANCE * f32::sin(p_euler.0);
//         transform.translation.z = p_trans.z + CAM_DISTANCE * f32::cos(p_euler.0);
//         transform.look_at(Vec3::new(p_trans.x, 0.0, p_trans.z), Vec3::Y);
//     }
// }

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
