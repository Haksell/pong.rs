use bevy::prelude::*;

fn spawn_ball(mut commands: Commands) {
    println!("Spawning ball...");
    commands.spawn_empty();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .run();
}
