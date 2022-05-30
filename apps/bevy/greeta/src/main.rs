use bevy::prelude::*;

// region:      --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;
// endregion:   --- Asset Constants

// region:      --- Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
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
        .add_startup_system(setup_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
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
    window.set_position(IVec2::new(800, 200));

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // setup a player
    let bottom = -win_size.h / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
// use For dev, fasterrecompile. (dynamic link bevy framework)
// `cargo run --release --features bevy/dynamic`
