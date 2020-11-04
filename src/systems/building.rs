// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

use crate::{systems::MouseInteractionEvent, Moon};

#[derive(Default)]
pub struct BuildingSystem {
    event_reader: Option<ReaderId<MouseInteractionEvent>>,
}

impl<'s> System<'s> for BuildingSystem {
    type SystemData = (
        WriteStorage<'s, Moon>,
        Read<'s, EventChannel<MouseInteractionEvent>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<MouseInteractionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            mut moons,
            event_channel,
        ): Self::SystemData,
    ) {
        for event in event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(moon) = moons.get_mut(event.entity) {
                moon.mining = !moon.mining;
            }
        }
    }
}
