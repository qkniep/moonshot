// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::{
    input::{keyboard::KeyboardInput, ElementState, Input},
    prelude::*,
};

use crate::{components::Moon, cursor_world_coords::*};

#[derive(Default)]
pub struct BuildingState {
    keyboard_event_reader: EventReader<KeyboardInput>,
    cursor_follower: Option<Entity>,
    currently_building: bool,
}

pub fn building(
    commands: &mut Commands,
    mut state: Local<BuildingState>,
    cursor_in_world: Res<CursorInWorld>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    mouse_input: Res<Input<MouseButton>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut moon_query: Query<(Mut<Moon>, Mut<TextureAtlasSprite>, &GlobalTransform)>,
) {
    let world_coords = cursor_in_world.position;

    // change to building mode on button press
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if event.key_code == Some(KeyCode::B) && event.state == ElementState::Pressed {
            state.currently_building = true;
            state.cursor_follower = commands
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(6),
                    texture_atlas: texture_atlases.get_handle("SPRITE_SHEET"),
                    transform: Transform {
                        translation: world_coords.extend(0.0),
                        rotation: Quat::default(),
                        scale: Vec3::splat(0.25),
                    },
                    ..Default::default()
                })
                .with(CursorFollowing)
                .current_entity();
        }
    }

    if state.currently_building {
        if mouse_input.pressed(MouseButton::Left) {
            // check if cursor is inside of a moon
            // TODO: use actual sprite size instead of magic number
            for (mut moon, mut sprite, trans) in moon_query.iter_mut() {
                if trans.translation.x() - 128.0 * trans.scale.x() <= world_coords.x()
                    && trans.translation.x() + 128.0 * trans.scale.x() >= world_coords.x()
                    && trans.translation.y() - 128.0 * trans.scale.y() <= world_coords.y()
                    && trans.translation.y() + 128.0 * trans.scale.y() >= world_coords.y()
                {
                    sprite.index = 4;
                    moon.mining = true;
                }
            }
            commands.despawn(state.cursor_follower.unwrap());
            state.currently_building = false;
        }
    }
}
