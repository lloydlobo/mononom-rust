use crate::{
    components::{Opponent, SpriteSize},
    GameTextures, WinSize, OPPONENT_SIZE, SPRITE_SCALE,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct OpponentPlugin;

impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, opponent_spawn_system);
    }
}

fn opponent_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // compute the x/y with random values
    let mut rng = thread_rng();
    let w_span = win_size.w / 2.0 - 100.0;
    let h_span = win_size.h / 2.0 - 100.0;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.opponent.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 10.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Opponent)
        .insert(SpriteSize::from(OPPONENT_SIZE));
}
