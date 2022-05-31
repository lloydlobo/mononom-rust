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

// region:      --- Explosion Components

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3); // where the explosion needs to spawn - triggering explosion vs without having to worry about where the position has to spawn. Triggering vs Executing

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, true))
    }
}
// endregion:   --- Explosion Components
