use std::{thread, time};

use boomerang_br::networking::{message::Message, server::Server};

const PORT: &str = "1729";

fn main() {
    let mut server = Server::new(PORT).unwrap();
    println!("{}", server.address().unwrap());
    loop {
        server.accept();

        // println!("Sending message...");
        server.send_all(&Message::Info {
            msg: "Hello World!".to_string(),
        });

        for m in server.receive_all() {
            println!("{:?}", m);
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
