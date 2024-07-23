use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 10.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
