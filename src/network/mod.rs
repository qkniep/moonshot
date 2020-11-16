// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod time;

use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use bevy::prelude::*;
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
            .add_system(update_simulation_time.system())
            .add_system(send_messages.system())
            .add_system(handle_messages.system());
    }
}

pub struct Message {
    length: u16,
    payload: Vec<u8>,
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
        stream
            .write_all(&[(message.length / 256) as u8, (message.length % 256) as u8])
            .expect("Failed to send length of network message~");
        if let Err(e) = stream.write_all(&message.payload) {
            eprintln!("Failed to send network message: {}", e);
        }
    }
}

fn handle_messages(
    mut stream: ResMut<TcpStream>,
    mut event_channel: ResMut<Events<NetworkSimulationEvent>>,
) {
    let peer_addr = stream.peer_addr().unwrap();

    if let Ok(turn) = bincode::deserialize_from(&mut *stream) {
        println!("Received msg: {:?}", turn);
        let x: ServerTurn = turn;
        //event_channel.send(NetworkSimulationEvent::Message(peer_addr, msg_payload));
    }
}
