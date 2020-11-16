// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::prelude::Vec2;
use bytes::BytesMut;
use tokio::{
    net::{tcp::OwnedReadHalf, TcpStream},
    prelude::*,
};

use moonshot::network::{PlayerAction, ServerTurn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:7777").await?;
    let (reader, mut writer) = stream.into_split();

    tokio::spawn(client_handler(reader));

    for i in 1..16u32 {
        let launch = PlayerAction::ShootRocket {
            pos: Vec2::splat(1.0 * i as f32),
            dir: Vec2::splat(99.9),
        };
        let serialized = bincode::serialize(&launch).unwrap();

        writer.write_u16(serialized.len() as u16).await?;
        writer.write_all(&serialized).await?;
    }
    writer.flush().await?;

    loop {}
}

async fn client_handler(mut reader: OwnedReadHalf) {
    let mut buf = BytesMut::with_capacity(65536);

    'outer: loop {
        match reader.read_buf(&mut buf).await {
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
            let turn = bincode::deserialize::<ServerTurn>(&bytes[2..]).unwrap();
            println!("{:?}", turn);
        }
    }
}
