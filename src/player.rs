use bevy::prelude::*;
use bevy_sprite3d::*;

const TURN_SPEED: f32 = 100.0;
const LIFT_SPEED: f32 = 18.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub turn_speed: f32,
    pub lift_speed: f32,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_params: Sprite3dParams,
) {
    let texture: Handle<Image> = asset_server.load("player.png");

    commands.spawn((
        Sprite3d {
            image: images.sprite.clone(),
            pixels_per_metre: 400.,
            partial_alpha: true,
            unlit: true,
            ..default() // pivot: Some(Vec2::new(0.5, 0.5)),
                        // double_sided: true,
        }
        .bundle(&mut sprite_params),
        Player {
            turn_speed: TURN_SPEED,
            lift_speed: LIFT_SPEED,
        },
        Name::new("Player"),
    ));

    // commands.spawn((SpriteBundle {
    //     texture,
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // },));
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, player)) = player_query.get_single_mut() else {
        return;
    };

    let turn_amount = player.turn_speed * time.delta_seconds();
    let lift_amount = player.lift_speed * time.delta_seconds();

    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y += lift_amount;
    }
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y -= lift_amount;
    }
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += turn_amount;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= turn_amount;
    }
}
