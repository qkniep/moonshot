// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::{
    input::{keyboard::KeyboardInput, ElementState, Input},
    prelude::*,
};

use crate::building::*;
use crate::components::*;
use crate::cursor_world_coords::*;
use crate::network::{PlayerAction, Transport};

#[derive(Default)]
pub struct CombatState {
    keyboard_event_reader: EventReader<KeyboardInput>,
    current_rocket_base: Option<Entity>,
}

/// System for shooting rockets in mouse cursor direction.
pub fn combat(
    commands: &mut Commands,
    mut state: Local<CombatState>,
    time: Res<Time>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    mouse_input: Res<Input<MouseButton>>,
    mut resources: ResMut<PlayerResources>,
    cursor_in_world: Res<CursorInWorld>,
    mut transport: ResMut<Transport>,
    moon_query: Query<(Entity, &Moon, &GlobalTransform)>,
    mut rocket_query: Query<(Entity, &Rocket, Mut<Transform>)>,
) {
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if event.key_code == Some(KeyCode::A) && event.state == ElementState::Pressed {
            if resources.pink < 3 || state.current_rocket_base.is_none() {
                continue;
            }
            resources.pink -= 3;

            let base_moon = state.current_rocket_base.unwrap();
            let (_, _, trans) = moon_query.get(base_moon).unwrap();
            let rocket_position = trans.translation;
            let rocket_direction =
                (cursor_in_world.position - trans.translation.truncate()).normalize();

            let launch = PlayerAction::ShootRocket {
                pos: rocket_position.truncate(),
                dir: rocket_direction,
            };
            let serialized = bincode::serialize(&launch).unwrap();
            transport.send(serialized);
        }
    }

    let world_coords = cursor_in_world.position;
    if mouse_input.pressed(MouseButton::Left) {
        // check if cursor is inside of a moon
        // TODO: use actual sprite size instead of magic number
        for (entity, moon, trans) in moon_query.iter() {
            if trans.translation.x - 128.0 * trans.scale.x <= world_coords.x
                && trans.translation.x + 128.0 * trans.scale.x >= world_coords.x
                && trans.translation.y - 128.0 * trans.scale.y <= world_coords.y
                && trans.translation.y + 128.0 * trans.scale.y >= world_coords.y
                && moon.building == Some(BuildingType::Production)
            {
                //sprite.index = ...;
                state.current_rocket_base = Some(entity);
            }
        }
    }

    // move rockets according to their current velocity
    for (entity, rocket, mut trans) in rocket_query.iter_mut() {
        trans.translation += rocket.velocity.extend(0.0) * time.delta_seconds;
        // despawn if out of bounds
        if trans.translation.length() > 2000.0 {
            commands.despawn(entity);
        }
    }
}
