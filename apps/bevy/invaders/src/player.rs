use crate::components::{Player, Velocity};
use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system);
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
                translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 + 5.0, 10.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity { x: 1.0, y: 0.0 });
}
