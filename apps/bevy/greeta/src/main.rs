// `cargo run --release --features bevy/dynamic`
#![allow(unused)] // silence unused warnings while exploring (to comment out)

pub(crate) use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide, utils::HashSet};
use components::{
    Explosion, ExplosionTimer, ExplosionToSpawn, FromOpponent, FromPlayer, Laser, Movable,
    Opponent, Player, SpriteSize, Velocity,
};
use opponent::OpponentPlugin;
use player::{player_restrict_win_edges, PlayerPlugin};

mod components;
mod opponent;
mod player;

// region:      --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (83.0, 75.0);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (17.0, 16.0);

const OPPONENT_SPRITE: &str = "opponent_a_01.png";
const OPPONENT_SIZE: (f32, f32) = (80.0, 72.0);
// const OPPONENT_SIZE: (f32, f32) = (144.0, 75.0);
const OPPONENT_LASER_SPRITE: &str = "laser_b_01.png";
const OPPONENT_LASER_SIZE: (f32, f32) = (17.0, 55.0);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

const SPRITE_SCALE: f32 = 0.5;

// endregion:   --- Asset Constants

// region:      --- Game Constants

const TIME_STEP: f32 = 1.0 / 60.0; // seconds per frame? (60 fps)? (1.0 / 60.0)
const BASE_SPEED: f32 = 500.0; // pixels per second

const PLAYER_RESPAWN_DELAY: f64 = 2.0;
const OPPONENT_MAX: u32 = 2;
const FORMATION_MEMBERS_MAX: u32 = 2;

// endregion:   --- Game Constants

// region:      --- Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    opponent: Handle<Image>,
    opponent_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

// endregion:   --- Resources
struct OpponentCount(u32);

/// Player State
/// # Description
/// This is the state of the player.
/// # Notes
/// This is a simple enum that is used to track the state of the player.
struct PlayerState {
    on: bool,       // alive
    last_shot: f64, // -1 if not shot / hit
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
        }
    }
}
impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }
    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}

/// # Main Application
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
        .add_plugin(OpponentPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_opponent_system)
        .add_system(opponent_laser_hit_player_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

/// # Setup System
/// This system is responsible for setting up the game.
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>, // mut windows: ResMut<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // set camera first
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // set windows
    let window = windows.get_primary_mut().unwrap(); // unwrap causes panic so use matching
    let (win_w, win_h) = (window.width(), window.height());

    // position window for now
    // window.set_position(IVec2::new(760, 200));

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        opponent: asset_server.load(OPPONENT_SPRITE),
        opponent_laser: asset_server.load(OPPONENT_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures); // it's done only one time
    commands.insert_resource(OpponentCount(0)); // what does the 0 mean? u32?
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        // region:      --- Movable Boundary Restrictions
        if movable.auto_despawn {
            //despawn when off screen
            const MARGIN: f32 = 200.0;
            if translation.y > win_size.h / 2.0 + MARGIN
                || translation.y < -win_size.h / 2.0 - MARGIN
                || translation.x > win_size.w / 2.0 + MARGIN
                || translation.x < -win_size.w / 2.0 - MARGIN
            {
                // println!("--> despawn, {entity:?}");
                commands.entity(entity).despawn();
            }
        }
        // endregion:   --- Movable Boundary Restrictions

        player_restrict_win_edges(&win_size, translation);
    }
}

fn player_laser_hit_opponent_system(
    mut commands: Commands,
    mut opponent_count: ResMut<OpponentCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    opponent_query: Query<(Entity, &Transform, &SpriteSize), With<Opponent>>,
) {
    // Book keeping or caching despawned entities is not recommended?
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate through lasers
    for (laser_entity, laser_transform, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = Vec2::from(laser_transform.scale.xy());

        // iterate through opponents
        for (opponent_entity, opponent_transform, opponent_size) in opponent_query.iter() {
            if despawned_entities.contains(&opponent_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            } // if opponent is despawned, skip - also avoids compiler warning

            let opponent_scale = Vec2::from(opponent_transform.scale.xy());

            // determine if there is a collision
            let collision = collide(
                laser_transform.translation, // laser position
                laser_size.0 * laser_scale,
                opponent_transform.translation, // laser position
                opponent_size.0 * laser_scale,  // why is this laser_scale? and not opponent_scale?
            );

            // perform collision logic
            if let Some(_) = collision {
                // remove the opponent after collision
                commands.entity(opponent_entity).despawn();
                despawned_entities.insert(opponent_entity);
                opponent_count.0 -= 1;

                // remove the laser which hit the opponent right after collision
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                //spawn the explosionToSpawn
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(opponent_transform.translation.clone()));
            } // we don't care about the data of the collision hence Some(_)
        }
    }
}

fn opponent_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromOpponent>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
    // we know there's only one player so..
    if let Ok((player_entity, player_transform, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_transform.scale.xy());

        for (laser_entity, laser_transform, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from(laser_transform.scale.xy());

            // determine if there is collision}
            let collision = collide(
                laser_transform.translation,
                laser_size.0 * laser_scale,
                player_transform.translation,
                player_size.0 * player_scale,
            );

            // perform collision logic
            if let Some(_) = collision {
                // remove the player after collision
                commands.entity(player_entity).despawn();
                player_state.shot(time.seconds_since_startup());

                // remove the laser which hit the player right after collision
                commands.entity(laser_entity).despawn();

                // spawn the explosionToSpawn
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(player_transform.translation.clone()));

                break;
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        // spawn the explosion sprite
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0, // no clone needed
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        // despawn the explosionToSpawn
        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to the next sprite frame cell/grid
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}

// #region:      --- ASSET_SERVER.LOAD()

// By default the ROOT is the directory of the Application, but this can be overridden by setting the "CARGO_MANIFEST_DIR" environment variable (see https://doc.rust-lang.org/cargo/reference/environment-variables.html) to another directory. When the application is run through Cargo, then "CARGO_MANIFEST_DIR" is automatically set to the root folder of your crate (workspace)

// endregion:   --- ASSET_SERVER.LOAD()
