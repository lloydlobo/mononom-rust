use bevy::prelude::*;
use std::any::type_name;

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
