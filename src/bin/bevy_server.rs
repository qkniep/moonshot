// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::io::Write;
use std::net::{TcpListener, TcpStream};

use bevy::prelude::*;

use moonshot::network::{ServerTurn, Transport};

fn main() {
    println!("Waiting for players...");
    let mut players = Vec::new();
    let mut listener = TcpListener::bind("127.0.0.1:7777").unwrap();
    handle_connects(&mut listener, &mut players, 2);
    println!("Found 2 players!");

    App::build()
        .add_resource(players)
        .add_plugins(MinimalPlugins)
        .add_plugin(ServerPlugin)
        .run();
}

struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Transport::default())
            //.add_resource(Events::<NetworkSimulationEvent>::default())
            //.add_resource(NetworkSimulationTime::default())
            //.add_system(update_simulation_time.system())
            .add_system(send_messages.system())
            .add_system(handle_messages.system());
    }
}

fn send_messages(mut transport: ResMut<Transport>, mut streams: ResMut<Vec<TcpStream>>) {
    let messages = transport.drain_messages();
    for message in messages {
        for stream in streams.iter_mut() {
            if let Err(e) = stream.write_all(&message.payload) {
                eprintln!("Failed to send network message: {}", e);
            }
        }
    }
}

fn handle_messages(mut streams: ResMut<Vec<TcpStream>>, mut transport: ResMut<Transport>) {
    for stream in streams.iter_mut() {
        if let Ok(action) = bincode::deserialize_from(&mut *stream) {
            println!("Received from client: {:?}", action);
            let msg = ServerTurn::new(vec![action]);
            let serialized = bincode::serialize(&msg).unwrap();
            transport.send(serialized);
        }
    }
}

fn handle_connects(listener: &mut TcpListener, streams: &mut Vec<TcpStream>, max_conns: usize) {
    for conn in listener.incoming() {
        if let Ok(stream) = conn {
            println!("Accepted connection!");
            stream.set_nonblocking(true).unwrap();
            streams.push(stream);
            if streams.len() >= max_conns {
                return;
            }
        }
    }
}
