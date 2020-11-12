// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::{io::{self, Write}, net::TcpStream};

use bevy::prelude::Vec2;

use moonshot::PlayerActions;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let launch = PlayerActions::ShootRocket { pos: Vec2::splat(1.0), vel: Vec2::splat(99.9) };

    let serialized = bincode::serialize(&launch).unwrap();
    stream.write(&[(serialized.len() / 256) as u8, (serialized.len() % 256) as u8])?;
    stream.write_all(&serialized)?;

    Ok(())
}
