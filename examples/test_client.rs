use std::{net::SocketAddr, thread, time};

use boomerang_br::networking::{client::Client, message::Message};

fn main() {
    let addr: SocketAddr = "127.0.0.1:1729".parse().unwrap();

    let mut client = Client::new(addr).unwrap();
    loop {
        match client.receive() {
            Some(msg) => {
                match msg {
                    Message::Info { msg } => println!("Message: {}", msg),
                    Message::PlayerUpdate { .. } => todo!(),
                };
            }
            None => {}
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
}
