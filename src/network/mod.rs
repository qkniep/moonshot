// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod time;

use std::{
    collections::VecDeque,
    io::Write,
    net::{SocketAddr, TcpStream},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::building::*;
use crate::components::{Aura, Moon, Rocket};
use self::time::*;

/// Player issued actions in the game which need to be processed through the server.
#[derive(Deserialize, Serialize, Debug)]
pub enum PlayerAction {
    Build { building: BuildingType, moon: u32 },
    ChangeAura { aura: Option<Aura>, planet: u32 },
    ShootRocket { pos: Vec2, dir: Vec2 },
}

/// A single frame of the server's simulation.
/// Contains a set of player issued actions which are executed on that frame of the simulation.
#[derive(Deserialize, Serialize, Debug)]
pub struct ServerTurn {
    actions: Vec<PlayerAction>,
}

impl ServerTurn {
    pub fn new(actions: Vec<PlayerAction>) -> Self {
        ServerTurn { actions }
    }
}

#[derive(Debug)]
pub enum NetworkSimulationEvent {
    Message(SocketAddr, Vec<u8>),
    Connect(SocketAddr),
    Disconnect(SocketAddr),
}

/// This plugin can be added into a Bevy app to add network functionality.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let stream = TcpStream::connect("127.0.0.1:7777").unwrap();
        stream.set_nonblocking(true).unwrap();
        app.add_resource(stream)
            .add_resource(Events::<NetworkSimulationEvent>::default())
            .add_resource(Transport::default())
            .add_resource(NetworkSimulationTime::default())
            .add_system(update_simulation_time)
            .add_system(send_messages)
            .add_system(handle_messages);
    }
}

pub struct Message {
    length: u16,
    pub payload: Vec<u8>,
}

#[derive(Default)]
pub struct Transport {
    messages: VecDeque<Message>,
}

impl Transport {
    pub fn send(&mut self, payload: Vec<u8>) {
        if payload.len() >= 65536 {
            panic!("Payload to large for u16 length field!");
        }

        self.messages.push_back(Message {
            length: payload.len() as u16,
            payload,
        });
    }

    pub fn drain_messages(&mut self) -> Vec<Message> {
        self.messages.drain(0..).collect()
    }
}

fn send_messages(mut transport: ResMut<Transport>, mut stream: ResMut<TcpStream>) {
    let messages = transport.drain_messages();
    for message in messages {
        if let Err(e) = stream.write_all(&message.payload) {
            error!("Failed to send network message: {}", e);
        }
    }
}

fn handle_messages(
    commands: &mut Commands,
    mut stream: ResMut<TcpStream>,
    mut event_channel: ResMut<Events<NetworkSimulationEvent>>,
    mut moon_query: Query<(Mut<Moon>, Mut<TextureAtlasSprite>)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let peer_addr = stream.peer_addr().unwrap();

    if let Ok(turn) = bincode::deserialize_from::<&mut TcpStream, ServerTurn>(&mut *stream) {
        trace!("Received msg: {:?}", turn);
        for action in turn.actions {
            match action {
                PlayerAction::Build { building, moon } => {
                    let (mut moon, mut sprite) = moon_query.get_mut(Entity::new(moon)).unwrap();
                    sprite.index = building_moon_texture_index(building);
                    moon.building = Some(building);
                },
                PlayerAction::ShootRocket { pos, dir } => {
                    let angle = dir.y.atan2(dir.x);
                    commands.spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(7),
                        texture_atlas: texture_atlases.get_handle("SPRITE_SHEET"),
                        transform: Transform {
                            translation: pos.extend(0.0),
                            rotation: Quat::from_rotation_z(angle),
                            scale: Vec3::splat(0.25),
                        },
                        ..Default::default()
                    })
                    .with(Rocket {
                        velocity: 300.0 * dir,
                    });
                }
                _ => {}
            }
        }
        //event_channel.send(NetworkSimulationEvent::Message(peer_addr, msg_payload));
    }
}
