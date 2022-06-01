use crate::{WinSize, BASE_SPEED, FORMATION_MEMBERS_MAX};
use bevy::prelude::Component;
use rand::{thread_rng, Rng};

/// Component - Opponent Formation (per opponent)
#[derive(Clone, Component)] // Clone the opponent
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32, // change per tick (in angle per second) - angle is in radians
}

/// Resource - Formation Maker
#[derive(Default)]
pub struct FormationMaker {
    current_template: Option<Formation>, // for all member of same formation
    current_members: u32,
}

/// Formation factory implementation
impl FormationMaker {
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        match (
            &self.current_template,
            self.current_members >= FORMATION_MEMBERS_MAX,
        ) {
            // if match has current template and still within max members
            (Some(template), false) => {
                self.current_members += 1;
                template.clone()
            }
            // if first formation or previous formation is full (ned to create a new one)
            (None, _) | (_, true) => {
                let mut rng = thread_rng();

                // compute the start x/y position
                let width_span = win_size.w / 2.0 + 100.0;
                let height_span = win_size.h / 2.0 + 100.0;
                let x = if rng.gen_bool(0.5) {
                    width_span
                } else {
                    -width_span
                };
                let y = rng.gen_range(-height_span..height_span) as f32;
                let start = (x, y);

                // compute the pivot x/y
                let width_span = win_size.w / 4.0;
                let height_span = win_size.h / 3.0 + 50.0;
                let pivot = (
                    rng.gen_range(-width_span..width_span),
                    rng.gen_range(0.0..height_span),
                );

                // compute the radius
                let radius = (rng.gen_range(80.0..150.0), 100.0); // hardcoded

                // compute the start angle
                // where the first spawn is to the center of the pivot
                let angle = (y - pivot.1).atan2(x - pivot.0);

                // speed (fixed for now)
                let speed = BASE_SPEED;

                // create the formation
                let formation = Formation {
                    start,
                    radius,
                    pivot,
                    speed,
                    angle,
                };

                // store as template
                self.current_template = Some(formation.clone());
                // reset members to 1
                self.current_members = 1;

                formation
            }
        }
    }
}
