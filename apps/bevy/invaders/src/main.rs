#![allow(unused)] // silence unused warnings while exploring the code (to comment out)

use bevy::prelude::*;

// region: --- Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png"; // Rust mascot
const PLAYER_SIZE: (f32, f32) = (144.0, 75.0); // now-> setup an asset_server
const SPRITE_SCALE: f32 = 0.5;
// endregion: --- Asset Constants

// region: --- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
}
// endregion: --- Resources

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
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
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

    // add WinSize resource
    let window_size = WinSize {
        w: window_width,
        h: window_height,
    };
    commands.insert_resource(window_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures); // it's done only one time
}

fn player_spawn_system(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    game_textures: Res<GameTextures>,
    window_size: Res<WinSize>,
) {
    // add Player
    let bottom = -window_size.h / 2.0;
    commands.spawn_bundle(SpriteBundle {
        // texture: asset_server.load(PLAYER_SPRITE),
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 + 5.0, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
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
// endregion: --- Archive - 3. Setup player bottom and

// region: --- Archive - 4. Setup player and scale
// texture: asset_server.load(PLAYER_SPRITE); -> done previously
// add in constants const SPRITE_SCALE: f32 = 0.5;
// transform: Transform {
// scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0);
// endregion: --- Archive - 4. Setup player and scale

// region: --- Archive - 5. Resource structs for resources
// THis is done to create another system for Player
// pub struct WinSize
// let window_size = WinSize { w: window_width, h: window_height };
//  commands.insert_resource(window_size);
// endregion: --- Archive - 5. Resource structs for resources

// region: --- Archive - 6. Setup player spawn system
// now add transform // after creating new system change to -window_size.h
// now .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)     --> only after the setup
// endregion: --- Archive - 6. Setup player spawn system

// region: --- Archive - 7. Add GameTextures resource
// let game_textures = GameTextures {
//     player: asset_server.load(PLAYER_SPRITE),
// };
// commands.insert_resource(game_textures); // it's done only one time
// endregion: --- Archive - 7. Add GameTextures resource

// endregion: --- ARCHIVE
