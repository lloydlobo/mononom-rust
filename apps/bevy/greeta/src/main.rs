#![allow(unused)] // silence unused warnings while exploring (to comment out)

use bevy::prelude::*;
use player::PlayerPlugin;

use components::{Player, Velocity};

mod components;
mod player;

// region:      --- Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (83., 75.);

const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (51., 48.);

const SPRITE_SCALE: f32 = 0.5;

// endregion:   --- Asset Constants

// region:      --- Game Constants

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;

// endregion:   --- Game Constants

// region:      --- Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}
// endregion:   --- Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Greeta Plays".to_string(),
            width: 509.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

// # Setup
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>, // mut windows: ResMut<Windows>,
    mut windows: ResMut<Windows>,
) {
    // set camera first
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // set windows
    let window = windows.get_primary_mut().unwrap(); // unwrap causes panic so use matching
    let (win_w, win_h) = (window.width(), window.height());

    // position window for now
    window.set_position(IVec2::new(760, 200));

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures); // it's done only one time
}

// use For dev, faster recompile. (dynamic link bevy framework)
// `cargo run --release --features bevy/dynamic`

// #region:      --- ASSET_SERVER.LOAD()
// By default the ROOT is the directory of the Application, but this can be overridden by setting the "CARGO_MANIFEST_DIR" environment variable (see https://doc.rust-lang.org/cargo/reference/environment-variables.html) to another directory. When the application is run through Cargo, then "CARGO_MANIFEST_DIR" is automatically set to the root folder of your crate (workspace)
