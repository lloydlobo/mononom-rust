use crate::components::{Player, Velocity};
use crate::{GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system);
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
        .insert(Player)
        .insert(Velocity { x: 0.0, y: 0.0 }); // make x = 0.0 from 1.0 as BaseSpeed increases speed
}

fn player_keyboard_event_system(
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

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
