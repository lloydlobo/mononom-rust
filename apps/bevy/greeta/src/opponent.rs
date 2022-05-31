use crate::{GameTextures, SPRITE_SCALE};
use bevy::prelude::*;

pub struct OpponentPlugin;

impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, opponent_spawn_system);
    }
}

fn opponent_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.opponent.clone(),
        transform: Transform {
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
