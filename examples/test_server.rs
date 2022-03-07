use std::{thread, time};

use boomerang_br::networking::{message::Message, server::Server};

fn main() {
    let mut server = Server::new().unwrap();
    println!("{}", server.address().unwrap());
    loop {
        server.accept();

        // println!("Sending message...");
        server.send_all(&Message::Info {
            msg: "Hello World!".to_string(),
        });

        thread::sleep(time::Duration::from_millis(1000));
    }
}
