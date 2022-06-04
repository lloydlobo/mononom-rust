// $ cargo run --bin render_2d --release --features bevy/dynamic
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use move_sprite::MovableSpriteLogoPlugin;
use rectangle::RectanglePlugin;

mod components;
mod move_sprite;
mod rectangle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup_system)
        .add_plugin(RectanglePlugin)
        .add_plugin(MovableSpriteLogoPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // region:             --- mesh2d
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(100.0)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..Default::default()
    });
    // endregion:          --- mesh2d
}
