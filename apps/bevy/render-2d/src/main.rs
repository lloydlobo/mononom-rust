// $ cargo run --bin render_2d --release --features bevy/dynamic
use bevy::prelude::*;
use rectangle::RectanglePlugin;

mod rectangle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup_system)
        .add_plugin(RectanglePlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
