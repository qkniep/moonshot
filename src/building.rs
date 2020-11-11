// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::{
    input::{keyboard::KeyboardInput, ElementState, Input},
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
    ui::camera::UI_CAMERA,
};

use crate::components::Moon;

#[derive(Default)]
pub struct BuildingState {
    keyboard_event_reader: EventReader<KeyboardInput>,
    cursor_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
    cursor_follower: Option<Entity>,
    currently_building: bool,
}

pub struct CursorFollowing;

pub fn building(
    commands: &mut Commands,
    mut state: Local<BuildingState>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    mouse_input: Res<Input<MouseButton>>,
    cursor_inputs: Res<Events<CursorMoved>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    camera_query: Query<(&Camera, &Transform, &OrthographicProjection)>,
    mut moon_query: Query<(Mut<Moon>, Mut<TextureAtlasSprite>, &GlobalTransform)>,
    mut sprite_query: Query<(&CursorFollowing, Mut<Transform>)>,
) {
    for event in state.cursor_event_reader.iter(&cursor_inputs) {
        state.cursor_position = event.position;
    }

    // get the releveant attributes of the 2D orth. projection
    let mut camera_pos = Vec2::splat(0.0);
    let mut camera_width = 0.0;
    let mut camera_height = 0.0;
    for (camera, trans, orth) in camera_query.iter() {
        if camera.name == Some(UI_CAMERA.to_string()) {
            continue;
        }

        camera_pos = Vec2::new(trans.translation.x(), trans.translation.y());
        camera_width = orth.right - orth.left;
        camera_height = orth.top - orth.bottom;
    }

    // convert cursor position to world coordinates
    let x = state.cursor_position.x();
    let y = state.cursor_position.y();
    let screen_coords = Vec2::new(x - camera_width / 2.0, y - camera_height / 2.0);
    let world_coords = camera_pos + screen_coords;

    // change to building mode on button press
    let ta_id = texture_atlases.ids().next().unwrap();
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if event.key_code == Some(KeyCode::B) && event.state == ElementState::Pressed {
            state.currently_building = true;
            state.cursor_follower = commands
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(6),
                    texture_atlas: texture_atlases.get_handle(ta_id),
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

        // make the building indicator follow the mouse cursor
        for (_, mut trans) in sprite_query.iter_mut() {
            trans.translation = world_coords.extend(0.0);
        }
    }
}
