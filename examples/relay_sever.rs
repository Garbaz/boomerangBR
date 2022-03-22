use std::{thread, time};

use boomerang_br::networking::server::Server;

const TICK_RATE : f32 = 60.;

fn main() {
    let mut server = Server::new().unwrap();
    println!("{}", server.address().unwrap());
    loop {
        server.accept();

        // // println!("Sending message...");
        // server.send_all(&Message::Info {
        //     msg: "Hello World!".to_string(),
        // });

        for m in server.receive_all() {
            println!("{:?}", m);
            server.send_all(&m);
        }
        thread::sleep(time::Duration::from_secs_f32(1. / TICK_RATE));
    }
}
