// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    ecs::*,
    renderer::{
        sprite::{SpriteRender, SpriteSheet},
        Transparent,
    },
    shrev::{EventChannel, ReaderId},
};

use crate::{sprites::SpriteResource, systems::MouseInteractionEvent, Planet, Rocket};

#[derive(Default)]
pub struct CombatSystem {
    event_reader: Option<ReaderId<MouseInteractionEvent>>,
}

impl<'s> System<'s> for CombatSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Planet>,
        WriteStorage<'s, Rocket>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
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
        (entities, planets, rockets, sprite_resource, lazy_update, event_channel): Self::SystemData,
    ) {
        for event in event_channel.read(self.event_reader.as_mut().unwrap()) {
            if planets.get(event.entity).is_some() {
                spawn_rocket(
                    &entities,
                    sprite_resource.sprite_sheet.clone(),
                    Vector3::new(100.0, 100.0, 0.0),
                    &lazy_update,
                );
            }
        }
    }
}

pub fn spawn_rocket(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let sprite_render = SpriteRender::new(sprite_sheet, 3);

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);
    local_transform.set_scale(Vector3::new(0.25, 0.25, 0.25));

    let enemy_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(Rocket)
        .with(local_transform)
        .with(Transparent)
        .build();

    enemy_entity
}
