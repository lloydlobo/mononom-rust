use bevy::prelude::Component;

#[derive(Component)]
pub struct Rectangle;

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
}
