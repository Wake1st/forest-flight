use bevy::prelude::*;
use bevy_sprite3d::*;
use std::f32::consts::PI;

use crate::{assets::ImageAssets, state::GameState};

const LIFT_SPEED: f32 = 18.0;
const PLAYER_SPEED: f32 = 4.2;
const ROTATION_SPEED: f32 = PI * 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Ready)));
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub turn_speed: f32,
    pub lift_speed: f32,
}

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
        Player {
            turn_speed: ROTATION_SPEED,
            lift_speed: LIFT_SPEED,
        },
        Name::new("Player"),
    ));
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, player)) = player_query.get_single_mut() else {
        return;
    };
    let movement_amount = PLAYER_SPEED * time.delta_seconds();
    let rotation_amount = player.turn_speed * time.delta_seconds();
    // let lift_amount = player.lift_speed * time.delta_seconds();

    let mut rotation_factor = 0.0;
    let mut movement_factor = Vec3::ZERO;

    // gather user inputs
    if input.pressed(KeyCode::KeyA) {
        rotation_factor += 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        rotation_factor -= 1.0;
    }

    // update the ship rotation around the Y axis (perpendicular to the 2D plane of the screen)
    transform.rotate_y(rotation_factor * rotation_amount);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let z_direction = transform.rotation * Vec3::Z;
    let x_direction = transform.rotation * Vec3::X;

    if input.pressed(KeyCode::ArrowUp) {
        movement_factor.z -= 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        movement_factor.z += 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        movement_factor.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        movement_factor.x += 1.0;
    }
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * movement_amount;
    // create the change in translation using the new movement direction and distance
    let mut translation_delta = Vec3::ZERO;
    translation_delta.z = movement_distance.dot(z_direction);
    translation_delta.x = movement_distance.dot(-x_direction);
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;
}
