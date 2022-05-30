use crate::components::{Movable, Player, Velocity};
use crate::{GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP};
use bevy::prelude::*;
use bevy::render::texture;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            // .add_system(player_movement_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    game_textures: Res<GameTextures>,
    window_size: Res<WinSize>,
) {
    // add Player
    let bottom = -window_size.h / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            // texture: asset_server.load(PLAYER_SPRITE),
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 - 15.0, 10.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Player)
        .insert(Velocity { x: 0.0, y: 0.0 }); // make x = 0.0 from 1.0 as BaseSpeed increases speed
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            // add offset to lasers to translate them to the right
            let x_offset = PLAYER_SIZE.0 / 2.0 * SPRITE_SCALE - 5.0; // now add x_offset to x in translation:

            // add closure to spawn two lasers, |f32| -> ()
            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15.0, 0.0),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0.0, y: 1.0 }); // to go up put y: 1
            };

            spawn_laser(x_offset); // laser right
            spawn_laser(-x_offset); // laser left
        }
    }
}

fn player_keyboard_event_system(
    // mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.0
        } else if kb.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        }
    }
}
