use crate::{
    components::{FromOpponent, Laser, Movable, Opponent, SpriteSize, Velocity},
    GameTextures, OpponentCount, WinSize, BASE_SPEED, OPPONENT_LASER_SIZE, OPPONENT_MAX,
    OPPONENT_SIZE, SPRITE_SCALE, TIME_STEP,
};
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use self::formation::{Formation, FormationMaker};

mod formation;
pub struct OpponentPlugin;

impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system_to_stage(StartupStage::PostStartup, opponent_spawn_system);
        // app.add_system(opponent_spawn_system); // multiple opponents at once
        app.insert_resource(FormationMaker::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(opponent_spawn_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(opponent_fire_criteria)
                    .with_system(opponent_fire_system),
            )
            .add_system(opponent_movement_system); // won't use movable_system as we want an advanced control;
    }
}

fn opponent_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut opponent_count: ResMut<OpponentCount>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
) {
    if opponent_count.0 < OPPONENT_MAX {
        // get formation and start x/y from FormationMaker
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

        // // compute the x/y with random values --> deprecated after using FormationMaker
        // let mut rng = thread_rng();
        // let w_span = win_size.w / 2.0 - 100.0;
        // let h_span = win_size.h / 2.0 - 100.0;
        // let x = rng.gen_range(-w_span..w_span);
        // let y = rng.gen_range(-h_span..h_span);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.opponent.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(SPRITE_SCALE * 2.0, SPRITE_SCALE * 2.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Opponent)
            .insert(formation)
            .insert(SpriteSize::from(OPPONENT_SIZE));

        opponent_count.0 += 1;
    }
}

fn opponent_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1.0 / 60.0) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn opponent_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    opponent_query: Query<&Transform, With<Opponent>>,
) {
    for &opponent_transform in opponent_query.iter() {
        // spawn enemy laser sprite
        let (x, y) = (
            opponent_transform.translation.x,
            opponent_transform.translation.y,
        );
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.opponent_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15.0, 0.0),
                    rotation: Quat::from_rotation_x(PI),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..Default::default()
                },

                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(OPPONENT_LASER_SIZE))
            .insert(FromOpponent)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0.0, y: -1.0 });
    }
}

fn opponent_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Formation), With<Opponent>>,
) {
    // need mutable formation system to change the angle at some point
    let now = time.seconds_since_startup() as f32;

    for (mut transform, mut formation) in query.iter_mut() {
        /// # Current position
        let (x_origin, y_origin) = (transform.translation.x, transform.translation.y); // current position

        /// # max distance
        let max_distance = TIME_STEP * formation.speed; // distance in pixels per second
                                                        // let max_distance = TIME_STEP * BASE_SPEED; // distance in pixels per second

        /// # Fixtures
        let dir: f32 = if formation.start.0 < 0.0 { 1.0 } else { -1.0 }; // 1 for counter clockwise, -1 clockwise
                                                                         // let dir: f32 = -1.0; // 1 for counter clockwise, -1 clockwise
                                                                         // let (x_pivot, y_pivot) = (0.0, 0.0); // pivot point
                                                                         // let (x_radius, y_radius) = (200.0, 130.0); // radius of circle
        let (x_pivot, y_pivot) = formation.pivot; // pivot point
        let (x_radius, y_radius) = formation.radius; // radius of circle

        /// # Compute next angle (based on time for now)
        let angle = formation.angle
            + dir * formation.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.0); // in radians
                                                                                       // let angle = dir * BASE_SPEED * TIME_STEP * now % 360.0 / PI; // in radians

        /// # Compute next position target x/y
        let x_destination = x_radius * angle.cos() + x_pivot;
        let y_destination = y_radius * angle.sin() + y_pivot;

        // println!("{:?}", (x_origin, y_origin, x_destination, y_destination));

        /// # Compute next position / distance
        let distance_x = x_origin - x_destination;
        let distance_y = y_origin - y_destination;
        let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();
        let distance_ratio = if distance != 0.0 {
            max_distance / distance
        } else {
            0.0
        };

        /// # Compute final x/y position / target
        let x = x_origin - distance_x * distance_ratio;
        let x = if distance_x > 0.0 {
            x.max(x_destination)
        } else {
            x.min(x_destination)
        };
        let y = y_origin - distance_y * distance_ratio;
        let y = if distance_y > 0.0 {
            y.max(y_destination)
        } else {
            y.min(y_destination)
        };

        // start rotating the formation angle only when the sprite is on or close to the ellipse
        // (to avoid rotation when the sprite is far away) --> helps avoid complex maths this early on
        if distance < max_distance * formation.speed / 2.0 {
            formation.angle = angle;
        }
        /// # Update position
        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);

        // translation.x += BASE_SPEED * TIME_STEP / 4.0; // ->> SLOW MO
        // translation.y += BASE_SPEED * TIME_STEP / 4.0; // ->> SLOW MO
    }
}
