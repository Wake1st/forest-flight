use bevy::{math::vec3, prelude::*};

use crate::{player::Player, state::GameState};

pub const GRAVITY: f32 = 0.6;
const GLIDE_MAGIC: f32 = 0.2;
const MAX_VELOCITY: Vec3 = vec3(5.0, 5.0, 145.0);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_gravity,
                update_velocity,
                dampen_velocity,
                update_position,
            )
                .chain()
                .run_if(in_state(GameState::Ready)),
        );
    }
}

#[derive(Bundle)]
pub struct KinematicBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

fn apply_gravity(mut player: Query<&mut Transform, With<Player>>) {
    let Ok(mut transform) = player.get_single_mut() else {
        return;
    };

    transform.translation.y -= GRAVITY * GLIDE_MAGIC;
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn dampen_velocity(mut query: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = query.get_single_mut() else {
        return;
    };
    velocity.value = velocity.value.min(MAX_VELOCITY).max(-MAX_VELOCITY);
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let rotation = transform.rotation;
        transform.translation += rotation * velocity.value * time.delta_seconds();
    }
}
