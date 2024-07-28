use bevy::prelude::*;
use bevy_sprite3d::*;

use crate::{assets::ImageAssets, movement::*, state::GameState};

// const LIFT_SPEED: f32 = 18.0;
// const GRAVITY: f32 = 0.0;
const ROTATION_SPEED: f32 = 0.1;
const PLAYER_SPEED: f32 = 0.2;
// const MAX_SPEED: Vec3 = vec3(5.0, 1.0, 5.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SpawnPlayer),
            (spawn_player, set_state_ready).chain(),
        )
        .add_systems(
            Update,
            (rotate_player, move_player)
                .chain()
                .run_if(in_state(GameState::Ready)),
        );
    }
}

#[derive(Component, Default)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    let atlas = TextureAtlas {
        layout: images.layout.clone(),
        index: (0) as usize,
    };

    commands.spawn((
        Sprite3d {
            image: images.image.clone(),
            pixels_per_metre: 16.,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
        .bundle_with_atlas(&mut sprite_params, atlas),
        KinematicBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
        },
        Player {},
        Name::new("Player"),
    ));
}

fn set_state_ready(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Ready);
}

//  update the player's rotation
fn rotate_player(
    mut player: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut transform = player.single_mut();

    let mut rotation_factor = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowLeft) {
        rotation_factor.z += ROTATION_SPEED;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        rotation_factor.z -= ROTATION_SPEED;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        rotation_factor.x -= ROTATION_SPEED;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        rotation_factor.x += ROTATION_SPEED;
    }

    if rotation_factor != Vec3::ZERO {
        transform.rotate_local_z(rotation_factor.z);
        transform.rotate_local_x(rotation_factor.x);
    }
}

//  update the movement of the player
fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut transform = player.single_mut();

    let mut movement_factor = Vec3::ZERO;

    if keyboard.pressed(KeyCode::Space) {
        movement_factor.z -= PLAYER_SPEED;
    }

    if movement_factor != Vec3::ZERO {
        let rotation = transform.rotation;
        transform.translation += rotation * movement_factor;
    }
}

// fn player_movement(
//     mut query: Query<(&mut Acceleration, &mut Transform), With<Player>>,
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let Ok((mut acceleration, mut transform)) = query.get_single_mut() else {
//         return;
//     };

//     //  start with gravity
//     acceleration.value.y -= GRAVITY;

//     // gather user inputs
//     let mut rotation_factor = 0.0;
//     let mut movement_factor = Vec3::ZERO;

//     if input.pressed(KeyCode::ArrowLeft) {
//         rotation_factor += 1.0;
//     }
//     if input.pressed(KeyCode::ArrowRight) {
//         rotation_factor -= 1.0;
//     }
//     if input.pressed(KeyCode::ArrowUp) {
//         movement_factor.z -= 1.0;
//     }
//     if input.pressed(KeyCode::ArrowDown) {
//         movement_factor.z += 1.0;
//     }

//     //  calculate the rotation of the player
//     let rotation_amount = ROTATION_SPEED * time.delta_seconds();
//     transform.rotate_y(rotation_factor * rotation_amount);

//     //  calculate the movement of the player
//     let movement_amount = PLAYER_SPEED * time.delta_seconds();
//     let movement_distance = movement_factor * movement_amount;

//     let z_direction = transform.rotation * Vec3::Z;
//     let x_direction = transform.rotation * Vec3::X;

//     let mut delta = Vec3::ZERO;
//     delta.z = movement_distance.dot(z_direction);
//     delta.x = movement_distance.dot(-x_direction);

//     // let mut momentum = Vec3::ZERO;
//     // momentum.z = acceleration.value.dot(z_direction);
//     // momentum.x = acceleration.value.dot(-x_direction);

//     acceleration.value += delta;
// }

// fn dampen_velocity(mut query: Query<&mut Velocity, With<Player>>) {
//     let Ok(mut velocity) = query.get_single_mut() else {
//         return;
//     };
//     velocity.value = velocity.value.min(MAX_SPEED).max(-MAX_SPEED);
// }
