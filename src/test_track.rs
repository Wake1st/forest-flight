use bevy::{math::vec3, prelude::*};
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use rand::prelude::*;

use crate::{assets::ImageAssets, state::GameState};

pub struct TestTrackPlugin;

const TRACK_POSITION: Vec3 = vec3(0., -20., -10.);
const TRACK_SPACING: f32 = 8.0;
const TRACK_BOUNDS: f32 = 3.4;
const TRACK_SECTIONS: usize = 16;
const SUBDIVISIONS: usize = 16;
const TILE_SIZE: f32 = 0.25;

impl Plugin for TestTrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SpawnPlayer),
            spawn_track.run_if(in_state(GameState::SpawnPlayer)),
        );
    }
}

struct TrackSection {
    location: Vec3,
}

fn spawn_track(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    let sections: &[TrackSection; TRACK_SECTIONS] = &[
        TrackSection {
            location: vec3(0., 5., 20.),
        },
        TrackSection {
            location: vec3(0., 0., 0.),
        },
        TrackSection {
            location: vec3(0., -5., -20.),
        },
        TrackSection {
            location: vec3(0., -10., -50.),
        },
        TrackSection {
            location: vec3(0., -15., -60.),
        },
        TrackSection {
            location: vec3(0., -20., -80.),
        },
        TrackSection {
            location: vec3(0., -25., -120.),
        },
        TrackSection {
            location: vec3(0., -20., -150.),
        },
        TrackSection {
            location: vec3(0., -25., -180.),
        },
        TrackSection {
            location: vec3(0., -30., -200.),
        },
        TrackSection {
            location: vec3(0., -35., -220.),
        },
        TrackSection {
            location: vec3(0., -35., -250.),
        },
        TrackSection {
            location: vec3(0., -40., -270.),
        },
        TrackSection {
            location: vec3(0., -45., -290.),
        },
        TrackSection {
            location: vec3(0., -50., -320.),
        },
        TrackSection {
            location: vec3(0., -50., -350.),
        },
    ];

    // Create a vector of positions for "railing"
    let mut rnd = rand::thread_rng();
    let mut gen_rand_point = |start: Vec3, end: Vec3| {
        vec3(
            start.x.lerp(end.x, rnd.gen()),
            start.y.lerp(end.y, rnd.gen()),
            start.z.lerp(end.z, rnd.gen()),
        )
    };

    let mut entry = gen_rand_point(sections[0].location, sections[1].location);

    let mut points: Vec<Vec3> = Vec::new();
    for i in 1..=(TRACK_SECTIONS - 2) {
        let start = sections[i].location;
        let end = sections[i + 1].location;
        let exit = gen_rand_point(start, end);

        let b_spline = CubicBezier::new([[start, entry, exit, end]; 1]).to_curve();
        let positions: Vec<Vec3> = b_spline.iter_positions(SUBDIVISIONS).collect();
        points.extend(positions);

        entry = end + end - exit;
    }

    let mut entity = |(x, y, z), tile_x, tile_y, height| {
        for i in 0usize..height {
            let atlas = TextureAtlas {
                layout: images.layout.clone(),
                index: (tile_x + (tile_y + i) * 2) as usize,
            };

            commands.spawn((
                Sprite3d {
                    image: images.image.clone(),
                    pixels_per_metre: 16.,
                    transform: Transform::from_xyz(
                        TRACK_SPACING * (TRACK_POSITION.x + x + TRACK_BOUNDS),
                        TRACK_SPACING * (TRACK_POSITION.y + y + TILE_SIZE * i as f32),
                        TRACK_SPACING * (TRACK_POSITION.z + z),
                    ),
                    ..default()
                }
                .bundle_with_atlas(&mut sprite_params, atlas.clone()),
                Name::new("TreePart"),
            ));
            commands.spawn((
                Sprite3d {
                    image: images.image.clone(),
                    pixels_per_metre: 16.,
                    transform: Transform::from_xyz(
                        TRACK_SPACING * (TRACK_POSITION.x + x - TRACK_BOUNDS),
                        TRACK_SPACING * (TRACK_POSITION.y + y + TILE_SIZE * i as f32),
                        TRACK_SPACING * (TRACK_POSITION.z + z),
                    ),
                    ..default()
                }
                .bundle_with_atlas(&mut sprite_params, atlas.clone()),
                Name::new("TreePart"),
            ));
        }
    };

    for p in points {
        entity((p.x, p.y, p.z), 1, 0, 4);
    }
}
