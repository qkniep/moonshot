// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

use crate::Moon;

/// This system is responsible for moving moons around their planets.
#[derive(SystemDesc)]
pub struct KeplerSystem;

impl<'s> System<'s> for KeplerSystem {
    type SystemData = (
        WriteStorage<'s, Moon>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut moons, mut transforms, time): Self::SystemData) {
        for (moon, transform) in (&mut moons, &mut transforms).join() {
            let t = time.absolute_time_seconds();
            let ds = moon.velocity as f64 * t;
            transform.set_translation_x(moon.orbit_radius * ds.cos() as f32);
            transform.set_translation_y(moon.orbit_radius * ds.sin() as f32);
        }
    }
}
