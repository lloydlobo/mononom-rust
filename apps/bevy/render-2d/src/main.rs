// $ cargo run --bin render_2d --release --features bevy/dynamic

use bevy::prelude::*;
use mesh2d::Mesh2dPlugin;
use move_sprite::MovableSpriteLogoPlugin;
use rectangle::RectanglePlugin;

mod components;
mod mesh2d;
mod move_sprite;
mod rectangle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup_system)
        .add_plugin(RectanglePlugin)
        .add_plugin(Mesh2dPlugin)
        .add_plugin(MovableSpriteLogoPlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
