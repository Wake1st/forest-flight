use bevy::{math::vec3, prelude::*};

pub struct TestTrackPlugin;

impl Plugin for TestTrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_track);
    }
}

fn spawn_track(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points: &[Vec3; 6] = &[
        vec3(0., 0., 0.),
        vec3(0., -5., -20.),
        vec3(0., -10., -50.),
        vec3(0., -15., -60.),
        vec3(0., -20., -80.),
        vec3(0., -25., -120.),
    ];

    let pink = Color::hsv(326.0, 1.0, 1.0);
    let green = Color::hsv(146.0, 1.0, 1.0);
    let mut flip_colors: bool = false;

    for p in points {
        let l_color: Color = if flip_colors { pink } else { green };
        let r_color: Color = if flip_colors { green } else { pink };

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::splat(1.0)))),
            material: materials.add(l_color),
            transform: Transform::from_xyz(p.x + 10.0, p.y, p.z),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::splat(1.0)))),
            material: materials.add(r_color),
            transform: Transform::from_xyz(p.x - 10.0, p.y, p.z),
            ..default()
        });

        println!(
            "flip {:?}\tleft {:?}\tright {:?}",
            flip_colors, l_color, r_color
        );
        flip_colors = !flip_colors;
    }
}
