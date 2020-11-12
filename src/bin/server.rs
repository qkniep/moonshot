// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use tokio::net::TcpListener;
use tokio::prelude::*;

use moonshot::PlayerActions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 65536];
            let mut read_so_far = 0;

            // In a loop, read data from the socket and write the data back.
            loop {
                read_so_far += match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                if read_so_far >= 2 {
                    let size = buf[0] as usize * 256 + buf[1] as usize;
                    if read_so_far - 2 < size {
                        continue;
                    }

                    println!("{:?}", bincode::deserialize::<PlayerActions>(&buf[2..2+size]).unwrap());

                    buf = [0; 65536];
                    read_so_far = 0;
                }
            }
        });
    }
}
