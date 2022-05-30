#![allow(unused)] // silence unused warnings while exploring the code

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Greeta Greets!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
   .add_plugins(DefaultPlugins
                )
       .run();
}

// region: --- Archive - 1. Setup rectangle mock

// sprite: Sprite {
//     color: Color::rgb(0.25, 0.25, 0.75),
//     custom_size: Some(Vec2::new(150.0, 150.0)),
//     ..Default::default() // now add_startup_system(setup_system) called once in the beginning and then later on we add commands to be called every frame
// },

// endregion: --- Archive - 1. Setup rectangle mock
