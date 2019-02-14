#[macro_use]
extern crate lazy_static;
extern crate  crossbeam;

use std::io::{Read,Write};
use std::thread;
use std::net::{Shutdown,TcpListener,TcpStream};

mod config;
mod works;
mod msg;
mod cmd;
mod init;
mod memodis;
mod dict;
mod obj;

fn main() {
    let app = config::read_config();
    msg::inital_message_channel(app.config.thread_limit);

    //initialize db
    init::init();

    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    works::works_start(app.config.thread_limit);
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("{:?}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            // drop(msg::ORDER_CHANNEL.0);
        }
    }
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0 as u8; 10240];

    while let Ok(size) = stream.read(&mut buf) {
        if size == 0 {break;}
        let clientmsg = msg::OrderChannelMsg::new(String::from_utf8(buf[0..size].to_vec()).unwrap(),stream.peer_addr().unwrap());
        msg::ORDER_CHANNEL.0.send(clientmsg).unwrap();
    }
    println!("this stream is stopping");
    stream.shutdown(Shutdown::Both).unwrap();
    stream.flush().unwrap();
}