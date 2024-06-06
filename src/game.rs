use bevy::prelude::*;

use crate::states::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(
                Update,
                playing_update.run_if(in_state(PauseState::Unpaused)),
            );
    }
}

pub fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        StateScoped(GameState::Playing),
        SpriteBundle {
            texture: asset_server.load("square.png"),
            ..default()
        },
    ));
}

pub fn playing_update(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * time.delta_seconds() * 300.0;
        }
    }
}
