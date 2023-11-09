// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, start_animation)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("thing.glb#Scene0"),
        ..Default::default()
    });
}

fn start_animation(
    keyboard: Res<Input<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    asset_server: Res<AssetServer>,
) {
    for keycode in keyboard.get_just_pressed() {
        let animation_clip = match keycode {
            KeyCode::Key1 => asset_server.load("thing.glb#Animation0"),
            KeyCode::Key2 => asset_server.load("thing.glb#Animation1"),
            _ => continue,
        };
        for mut animation_player in animation_players.iter_mut() {
            animation_player
                .start(animation_clip.clone())
                .set_speed(1.0);
            if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
                animation_player.repeat();
            }
        }
    }
}
