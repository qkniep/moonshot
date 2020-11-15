// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod time;

use std::{
    collections::VecDeque,
    io::Write,
    net::{SocketAddr, TcpStream},
};

use bevy::prelude::*;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use time::*;

/// Player issued actions in the game which need to be processed through the server.
#[derive(Deserialize, Serialize, Debug)]
pub enum PlayerAction {
    Build(),
    ShootRocket { pos: Vec2, vel: Vec2 },
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
    Message(SocketAddr, Bytes),
    Connect(SocketAddr),
    Disconnect(SocketAddr),
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let stream = TcpStream::connect("127.0.0.1:7777").unwrap();
        app.add_resource(stream)
            .add_resource(Transport::default())
            .add_resource(NetworkSimulationTime::default())
            .add_system(update_simulation_time.system())
            .add_system(send_messages.system())
            .add_system(handle_messages.system());
    }
}

struct Message {
    length: u16,
    payload: Bytes,
}

#[derive(Default)]
struct Transport {
    messages: VecDeque<Message>,
}

impl Transport {
    pub fn send(&mut self, payload: Bytes) {
        if payload.len() >= 65536 {
            panic!("Payload to large for u16 length field!");
        }

        let message = Message {
            length: payload.len() as u16,
            payload,
        };
        self.messages.push_back(message);
    }

    /// Drains the messages queue and returns the drained messages.
    pub fn drain_messages(&mut self) -> Vec<Message> {
        self.messages.drain(0..).collect()
    }
}

fn handle_messages() {}

fn send_messages(
    mut transport: ResMut<Transport>,
    mut stream: ResMut<TcpStream>,
    sim_time: Res<NetworkSimulationTime>,
) {
    let messages = transport.drain_messages();
    for message in messages {
        if let Err(e) = stream.write(&message.payload) {
            eprintln!("Failed to send network message: {}", e);
        }
    }
}
