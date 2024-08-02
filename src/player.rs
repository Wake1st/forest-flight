use bevy::prelude::*;
use bevy_sprite3d::*;

use crate::{assets::SmallImageAssets, movement::*, state::GameState};

// const LIFT_SPEED: f32 = 18.0;
const ROTATION_SPEED: f32 = 0.04;
// const PLAYER_SPEED: f32 = 0.2;
const UPWARD_THRUST_BOOST: f32 = 4.0;

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
    images: Res<SmallImageAssets>,
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
    mut player: Query<(&Transform, &mut Velocity), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((transform, mut velocity)) = player.get_single_mut() else {
        return;
    };

    //  the velocity must be based on how "downward" the player is moving
    let downward_amount = transform.forward().dot(Vec3::Y);
    velocity.value.z += downward_amount * GRAVITY;

    //  player must be able to lift themselves (just a debug, or a feature?)
    if keyboard.pressed(KeyCode::Space) {
        velocity.value.z -= GRAVITY * UPWARD_THRUST_BOOST;
    }
}
