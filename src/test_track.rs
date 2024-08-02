use bevy::{math::vec3, prelude::*};
use rand::prelude::*;

pub struct TestTrackPlugin;

const TRACK_POSITION: Vec3 = vec3(0., -20., -10.);
const TRACK_SPACING: f32 = 8.0;
const TRACK_BOUNDS: f32 = 3.4;
const TRACK_SECTIONS: usize = 15;
const SUBDIVISIONS: usize = 16;

impl Plugin for TestTrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_track);
    }
}

struct TrackSection {
    location: Vec3,
}

fn spawn_track(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sections: &[TrackSection; TRACK_SECTIONS] = &[
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
    let mut points: Vec<Vec3> = Vec::new();
    let mut rnd = rand::thread_rng();
    for i in 0..=(TRACK_SECTIONS - 2) {
        let start = sections[i].location;
        let end = sections[i + 1].location;
        let middle = vec3(
            start.x.lerp(end.x, rnd.gen()),
            start.y.lerp(end.y, rnd.gen()),
            start.z.lerp(end.z, rnd.gen()),
        );
        let b_spline = CubicBezier::new([[start, middle, middle, end]; 1]).to_curve();
        let positions: Vec<Vec3> = b_spline.iter_positions(SUBDIVISIONS).collect();
        points.extend(positions);
    }

    let pink = Color::hsv(326.0, 1.0, 1.0);
    let green = Color::hsv(146.0, 1.0, 1.0);
    let mut flip_colors: bool = false;

    for p in points {
        let l_color: Color = if flip_colors { pink } else { green };
        let r_color: Color = if flip_colors { green } else { pink };

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::splat(1.0)))),
            material: materials.add(l_color),
            transform: Transform::from_xyz(
                TRACK_SPACING * (TRACK_POSITION.x + p.x + TRACK_BOUNDS),
                TRACK_SPACING * (TRACK_POSITION.y + p.y),
                TRACK_SPACING * (TRACK_POSITION.z + p.z),
            ),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::splat(1.0)))),
            material: materials.add(r_color),
            transform: Transform::from_xyz(
                TRACK_SPACING * (TRACK_POSITION.x + p.x - TRACK_BOUNDS),
                TRACK_SPACING * (TRACK_POSITION.y + p.y),
                TRACK_SPACING * (TRACK_POSITION.z + p.z),
            ),
            ..default()
        });

        flip_colors = !flip_colors;
    }
}
