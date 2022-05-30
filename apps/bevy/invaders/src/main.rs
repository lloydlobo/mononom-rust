#![allow(unused)] // silence unused warnings while exploring the code (to comment out)

use bevy::prelude::*;
use components::{Movable, Player, Velocity};
use player::PlayerPlugin;

mod components;
mod player;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png"; // Rust mascot
const PLAYER_SIZE: (f32, f32) = (144.0, 75.0); // now-> setup an asset_server
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9.0, 54.0);

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants

// region: --- Game Constants

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;

// endregion: --- Game Constants

// region: --- Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
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
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        // .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system) // remove it after importing PlayerPlugin as it's already in the stage
        .add_system(movable_system)
        .run();
}

// first system - 'setup system is a convention
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
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures); // it's done only one time
}

fn movable_system(
    mut commands: Commands,
    window_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        // check if movable (laser here) is out of screen
        if movable.auto_despawn {
            // despawn when out of screen
            const MARGIN: f32 = 200.0;
            if translation.y > window_size.h / 2.0 + MARGIN
                || translation.y < -window_size.h / 2.0 - MARGIN
                || translation.x > window_size.w / 2.0 + MARGIN
                || translation.x < -window_size.w / 2.0 - MARGIN
            {
                // println!("->> despawn {entity:?}");
                commands.entity(entity).despawn();
            }
        }
    }
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
// texture: game_textures.player.clone(),
// game_textures: Res<GameTextures>,

// endregion: --- Archive - 7. Add GameTextures resource

// region: --- Archive - 8. Setup keyboard_event_system (player_move_system)

//     if let Ok(mut velocity) = query.get_single_mut() {
//         velocity.x = if kb.pressed(KeyCode::Left) {
//             -1.0} else if kb.pressed(KeyCode::Right) {1.0} else {0.0}; } } }

// endregion: --- Archive - 8. Setup keyboard_event_system (player_move_system)

// region: --- Archive - 9. Setup player_fire_system player_laser_sprite and use Space key to fire laser
// texture: asset_server.load(PLAYER_LASER_SPRITE);
// endregion: --- Archive - 9. Setup player_fire_system player_laser_sprite and use Space key to fire laser

// region: --- Archive - 10. Add velocity, movable component & refactor player_movement_system to movable_system here

// fn movable_system(
//     mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
// ) {
//     for (entity, velocity, mut transform, movable) in query.iter_mut() {
//
// add movable_system in main()
// in fn main() {
// .add_system(movable_system)

// endregion: --- Archive - 10. Add velocity, movable component & refactor player_movement_system to movable_system here

// endregion: --- ARCHIVE
