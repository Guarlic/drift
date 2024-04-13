use bevy::prelude::*;

mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "DRIFT".into(),
                resolution: (1920., 1080.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup, (
                systems::spawn_camera,
                systems::spawn_player
            )
        )
        .add_systems(
            Update, (
                systems::move_player,
            )
        )
        .run();
}