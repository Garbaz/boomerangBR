use std::{net::SocketAddr, thread, time};

use boomerang_br::networking::{client::Client, message::Message};

fn main() {
    let addr: SocketAddr = "127.0.0.1:1729".parse().unwrap();

    let mut client = Client::new(addr).unwrap();
    loop {
        match client.receive() {
            Some(msg) => {
                println!("{:?}", msg)
            }
            None => {}
        }

        client.send(&Message::PlayerUpdate {
            id: 0,
            pos: (17., 12.),
            rotation: 0.,
            vel: (-100., -75.),
        });

        client.send(&Message::BoomerangUpdate {
            id: 1,
            pos: (42., 17.),
            rotation: 231.,
            vel: (1., 2.),
        });

        thread::sleep(time::Duration::from_millis(1000));
        println!("...")
    }
}
