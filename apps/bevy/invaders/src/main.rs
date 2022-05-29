#![allow(unused)] // silence unused warnings while exploring the code (to comment out)

use bevy::prelude::*;

// region: --- Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png"; // Rust mascot
const PLAYER_SIZE: (f32, f32) = (144.0, 75.0); // now-> setup an asset_server

// endregion: --- Asset Constants

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
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (window_width, window_height) = (window.width(), window.height()); // now set bottom

    // position window (for tutorial)
    window.set_position(IVec2::new(600, 0));

    // add Player
    let bottom = -window_height / 2.0; // now add transform
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 + 5.0, 10.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

// region: --- ARCHIVE

// region: --- Archive - 1. Setup rectangle mock
// sprite: Sprite {
//     color: Color::rgb(0.25, 0.25, 0.75),
//     custom_size: Some(Vec2::new(150.0, 150.0)),
//     ..Default::default() // now add_startup_system(setup_system) called once in the beginning and then later on we add commands to be called every frame
// },
// endregion: --- Archive - 1. Setup rectangle mock

// region: --- Archive - 2. Setup mutable window
/* usually he prefers not to use unwrap */
// endregion: --- Archive - 2. Setup mutable window

// region: --- Archive - 3. Setup player bottom and transform
// let bottom = -window_height / 2.0; // now add transform
// endregion: --- Archive - 3. Setup player bottom and transform

// endregion: --- ARCHIVE
