use bevy::prelude::*;

// region:      --- Common Components

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool, // player won't despawn when they go off screen but laser will
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

// endregion:   --- Common Components

// region:      --- Player Components

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

// endregion:   --- Player Components

// region:      --- Opponent Components

#[derive(Component)]
pub struct Opponent;

#[derive(Component)]
pub struct FromOpponent;

// endregion:   --- Opponent Components
