use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::any::type_name;

pub struct Mesh2dPlugin;

impl Plugin for Mesh2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mesh2d_spawn_system);
    }

    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

fn mesh2d_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(100.0)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..Default::default()
    });
}
