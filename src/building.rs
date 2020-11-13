// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::{
    input::{keyboard::KeyboardInput, ElementState, Input},
    prelude::*,
};

use crate::{components::Moon, cursor_world_coords::*, PlayerResources};

#[derive(Clone, Copy)]
pub enum BuildingType {
    Mining,
    Production,
}

#[derive(Default)]
pub struct BuildingState {
    keyboard_event_reader: EventReader<KeyboardInput>,
    cursor_follower: Option<Entity>,
    currently_building: Option<BuildingType>,
}

pub fn building(
    commands: &mut Commands,
    mut state: Local<BuildingState>,
    cursor_in_world: Res<CursorInWorld>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    mouse_input: Res<Input<MouseButton>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut resources: ResMut<PlayerResources>,
    mut moon_query: Query<(Mut<Moon>, Mut<TextureAtlasSprite>, &GlobalTransform)>,
) {
    let world_coords = cursor_in_world.position;

    // change to building mode on button press
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if state.currently_building.is_none() && event.state == ElementState::Pressed {
            state.currently_building = match event.key_code {
                Some(KeyCode::B) => Some(BuildingType::Mining),
                Some(KeyCode::R) => Some(BuildingType::Production),
                _ => state.currently_building,
            };

            if let Some(building) = state.currently_building {
                state.cursor_follower = commands
                    .spawn(SpriteSheetComponents {
                        sprite: building_cursor_texture(building),
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
    }

    if let Some(building) = state.currently_building {
        if mouse_input.pressed(MouseButton::Left) {
            // check if cursor is inside of a moon
            // TODO: use actual sprite size instead of magic number
            for (mut moon, mut sprite, trans) in moon_query.iter_mut() {
                if trans.translation.x() - 128.0 * trans.scale.x() <= world_coords.x()
                    && trans.translation.x() + 128.0 * trans.scale.x() >= world_coords.x()
                    && trans.translation.y() - 128.0 * trans.scale.y() <= world_coords.y()
                    && trans.translation.y() + 128.0 * trans.scale.y() >= world_coords.y()
                    && resources.pink >= building_cost(building)
                {
                    sprite.index = building_moon_texture_index(building);
                    moon.building = Some(building);
                    resources.pink -= building_cost(building);
                }
            }
            commands.despawn(state.cursor_follower.unwrap());
            state.currently_building = None;
        }
    }
}

fn building_cursor_texture(building: BuildingType) -> TextureAtlasSprite {
    match building {
        BuildingType::Mining => TextureAtlasSprite::new(6),
        BuildingType::Production => TextureAtlasSprite::new(5),
    }
}

fn building_moon_texture_index(building: BuildingType) -> u32 {
    match building {
        BuildingType::Mining => 4,
        BuildingType::Production => 8,
    }
}

fn building_cost(building: BuildingType) -> u32 {
    match building {
        BuildingType::Mining => 20,
        BuildingType::Production => 15,
    }
}
