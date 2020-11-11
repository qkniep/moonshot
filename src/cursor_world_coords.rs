// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::{
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
    ui::camera::UI_CAMERA,
};

pub struct CursorFollowing;

#[derive(Debug, Default, Copy, Clone)]
pub struct CursorInWorld {
    pub position: Vec2,
}

#[derive(Default)]
pub struct CursorState {
    cursor_event_reader: EventReader<CursorMoved>,
}

pub fn cursor_world_coords(
    mut state: Local<CursorState>,
    mut cursor_in_world: ResMut<CursorInWorld>,
    cursor_inputs: Res<Events<CursorMoved>>,
    camera_query: Query<(&Camera, &Transform, &OrthographicProjection)>,
    mut cursorfollowing_query: Query<(&CursorFollowing, Mut<Transform>)>,
) {
    if let Some(event) = state.cursor_event_reader.iter(&cursor_inputs).last() {
        let cursor_position = event.position;

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
        let x = cursor_position.x();
        let y = cursor_position.y();
        let screen_coords = Vec2::new(x - camera_width / 2.0, y - camera_height / 2.0);
        let world_coords = camera_pos + screen_coords;

        // assign the new world coords to the gloabl resource
        cursor_in_world.position = world_coords;

        // move CursorFollowing entities to mouse cursor
        for (_, mut trans) in cursorfollowing_query.iter_mut() {
            trans.translation = world_coords.extend(0.0);
        }
    }
}
