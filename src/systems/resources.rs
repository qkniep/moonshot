// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::{Moon, Planet, ResourcesBoard};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{Entity, Join, Read, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
};

/// This system is responsible for counting the resources harvested by the player
#[derive(SystemDesc)]
pub struct ResourcesSystem;

impl<'s> System<'s> for ResourcesSystem {
    type SystemData = (
        WriteStorage<'s, Moon>,
        WriteStorage<'s, Planet>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ResourcesBoard>,
        ReadExpect<'s, ResourcesText>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            mut moons,
            mut planets,
            mut transforms,
            mut text,
            mut resources_board,
            resources_text,
            time,
        ): Self::SystemData,
    ) {
        for (moon, transform) in (&mut moons, &mut transforms).join() {
            //let dt = time.delta_seconds();
            //transform.prepend_translation_x(500. * (moon.velocity * dt).cos());
            //transform.prepend_translation_y(500. * (moon.velocity * dt).sin());
            let t = time.absolute_time_seconds();
            transform.set_translation_x(1200. + 500. * (moon.velocity as f64 * t).cos() as f32);
            transform.set_translation_y(675. + 500. * (moon.velocity as f64 * t).sin() as f32);
        }
        //for (planet, transform) in (&mut planets, &mut transforms).join() {
        resources_board.red = (resources_board.red + 1).min(512);
        if let Some(text) = text.get_mut(resources_text.p_red) {
            text.text = resources_board.red.to_string();
        }
        //}
    }
}

/// Stores the entities that are displaying the player's resource count with UiText.
pub struct ResourcesText {
    pub p_red: Entity,
}
