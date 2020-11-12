// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::sync::Arc;

use bytes::BytesMut;
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    prelude::*,
    sync::{mpsc, Mutex},
};

use moonshot::{PlayerAction, ServerTurn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let players = Arc::new(Mutex::new(Vec::new()));
    let (sender, receiver) = mpsc::unbounded_channel();

    tokio::spawn(game_loop(receiver, players.clone()));

    let listener = TcpListener::bind("127.0.0.1:7777").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let (reader, writer) = socket.into_split();
        players.lock().await.push(writer);
        tokio::spawn(client_handler(reader, sender.clone()));
    }
}

async fn game_loop(
    mut receiver: mpsc::UnboundedReceiver<PlayerAction>,
    players: Arc<Mutex<Vec<OwnedWriteHalf>>>,
) {
    let mut actions_buffer = Vec::new();

    loop {
        if let Some(action) = receiver.recv().await {
            println!("{:?}", action);
            actions_buffer.push(action);
        }

        if actions_buffer.len() > 3 {
            let msg = ServerTurn::new(actions_buffer.drain(1..).collect());
            let serialized = bincode::serialize(&msg).unwrap();

            for player in players.lock().await.iter_mut() {
                println!(
                    "writing turn ({} bytes): {:?}",
                    serialized.len(),
                    serialized.as_slice()
                );
                player.write_u16(serialized.len() as u16).await.unwrap();
                player.write_all(&serialized).await.unwrap();
                player.flush().await.unwrap();
            }

            actions_buffer.clear();
        }
    }
}

async fn client_handler(mut socket: OwnedReadHalf, sender: mpsc::UnboundedSender<PlayerAction>) {
    let mut buf = BytesMut::with_capacity(65536);

    'outer: loop {
        match socket.read_buf(&mut buf).await {
            Ok(n) if n == 0 => return, // socket closed
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read from socket: {:?}", e);
                return;
            }
        };

        while buf.len() >= 2 {
            let size = buf[0] as usize * 256 + buf[1] as usize;
            if buf.len() - 2 < size {
                continue 'outer;
            }

            let bytes = buf.split_to(2 + size);
            let action = bincode::deserialize::<PlayerAction>(&bytes[2..]).unwrap();
            sender
                .send(action)
                .expect("Failed to send action to game_loop via channel.");
        }
    }
}
