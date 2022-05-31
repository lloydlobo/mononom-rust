use crate::components::{Movable, Player, Velocity};
use crate::{GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP};
use bevy::{prelude::*, window};
// use bevy::render::texture;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            // .add_system(player_movement_system) // moving the system to main.rs
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // setup a player
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        }) // this is a component, add more with chaining insert
        .insert(Player)
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity { x: 0.0, y: 0.0 }); // replace x: 1.0 with x: 0.0 as keyboard player start at rest
}

fn player_fire_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = query.get_single() {
        if keyboard.just_pressed(KeyCode::Space) {
            let (x, y) = (
                player_transform.translation.x,
                player_transform.translation.y,
            );

            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Movable { auto_despawn: true })
                .insert(Velocity { x: 0.0, y: 1.0 }); // similar to player movable system velocity but y is 1.0
        }
    }
}

// Magic of Bevy: Decouple the movement from the rendering
fn player_keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    // when you know there is a single playable entity use this method
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };
    }
    // for velocity in &mut query.iter() {}
}

// restrict player to window edges
pub(crate) fn player_restrict_win_edges(win_size: &Res<WinSize>, translation: &mut Vec3) {
    let win_edge = (win_size.w - PLAYER_SIZE.0 / 2.) * SPRITE_SCALE;
    if translation.x < -win_edge {
        translation.x = -win_edge;
    } else if translation.x > win_edge {
        translation.x = win_edge;
    }
}
