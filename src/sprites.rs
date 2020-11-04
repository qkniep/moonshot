// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{assets::Handle, ecs::prelude::World, renderer::SpriteSheet};

#[derive(Clone)]
pub struct SpriteResource {
    pub sprite_sheet: Handle<SpriteSheet>,
}

pub fn init_sprite_resource(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
) -> SpriteResource {
    let sprite_resource = SpriteResource {
        sprite_sheet: sprite_sheet_handle,
    };
    world.insert(sprite_resource.clone());
    sprite_resource
}
