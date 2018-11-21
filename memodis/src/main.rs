#[macro_use]
extern crate lazy_static;
extern crate  crossbeam;

use std::io::Read;
use std::io::Write;
use std::thread;
use std::net::{TcpListener, TcpStream};

mod config;
mod works;
mod msg;

fn main() {
    let app = config::read_config();
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    works::works_start(app.config.thread_limit);
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("{:?}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 4096];
        stream.read(&mut buf).unwrap();

        let command = String::from_utf8(buf[..].to_vec()).unwrap();
        msg::order_channel.0.send(command).unwrap();

        stream.flush().unwrap();
    }
}