#![allow(unused)] // silence unused warnings while exploring the code (to comment out)

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

// first system - 'setup_system is a convention
/// # Setup
/// This system is run once at the start of the game
/// It sets up the game state
/// System doesn't return any value
/// # Commands
/// This system runs the following commands:
/// - commands allow you to add / remove / modify entities
fn setup_system(mut commands: Commands) {
    // camera - 2D game
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // add rectangle entity to the scene
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(150.0, 150.0)),
            ..Default::default() // now add_startup_system(setup_system) called once in the beginning and then later on we add commands to be called every frame
        },
        ..Default::default()
    });
}
