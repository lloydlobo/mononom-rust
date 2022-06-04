use crate::components::Direction;
use bevy::{ecs::schedule::ShouldRun, prelude::*};
use rand::{thread_rng, Rng};
use std::any::type_name;
pub struct MovableSpriteLogoPlugin;

impl Plugin for MovableSpriteLogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_sprite_spawn_system).add_system_set(
            SystemSet::new()
                .with_run_criteria(sprite_movement_criteria)
                .with_system(sprite_movement_system),
        );
    }
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

fn move_sprite_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player_a_01.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Direction::Up);
}

fn sprite_movement_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1.0 / 60.0) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn sprite_movement_system(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut logo, mut transform) in sprite_position.iter_mut() {
        match *logo {
            Direction::Up => transform.translation.y += 150.0 * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150.0 * time.delta_seconds(),
        }
        if transform.translation.y > 200.0 {
            *logo = Direction::Down;
        } else if transform.translation.y < -200.0 {
            *logo = Direction::Up;
        }
    }
}
