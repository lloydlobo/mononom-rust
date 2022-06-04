// $ cargo run --bin render_2d --release --features bevy/dynamic
use bevy::prelude::*;
use std::any::type_name;

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

pub struct RectanglePlugin;

impl Plugin for RectanglePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, rectangle_system);
    }
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// This system will draw a rectangle
/// It will be added to the `PostStartup` stage
fn rectangle_system(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::hsl(156.0, 0.60, 0.60),
            custom_size: Some(Vec2::new(200.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
