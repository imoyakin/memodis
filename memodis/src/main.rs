#![feature(nll)]
#[macro_use]
extern crate lazy_static;
extern crate  crossbeam;

use std::io::{Read,Write};
use std::thread;
use std::net::{Shutdown, TcpListener,TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::rc::Rc;

mod config;
mod works;
mod cmd;
mod init;
mod model;

fn main() {
    let app = config::read_config();
    model::msg::inital_message_channel(app.config.thread_limit);
    works::works_start(app.config.thread_limit);
    //initialize db
    init::init(app.config);

    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("????{:?}", e),
            Ok(stream) => {
                let thread = thread::spawn(move || {
                    handle_client(stream);
                });

                //thread.join().ok();
            },
            // drop(msg::ORDER_CHANNEL.0);
        }
    }
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0 as u8; 10240];
    let s = Arc::new(Mutex::new(stream));
    while let Ok(size) = s.lock().unwrap().read(&mut buf) {
        if size == 0 {break;}
        let clientmsg = model::msg::OrderChannelMsg::new(
            String::from_utf8(buf[0..size].to_vec()).unwrap(), 
            s.clone());
        model::msg::ORDER_CHANNEL.0.send(clientmsg).unwrap();
    }
    // while let Ok(size) = s.write(&mut buf).unwrap() {
    //     if size == 0 {break;}
    //     let clientmsg = model::msg::OrderChannelMsg::new(
    //         String::from_utf8(buf[0..size].to_vec()).unwrap(), 
    //         s.clone());
    //     model::msg::ORDER_CHANNEL.0.send(clientmsg).unwrap();
    // }

    println!("this stream is stopping");
    // let mut l = s.lock().unwrap();
    // l.shutdown(Shutdown::Both).unwrap();
    // l.flush().unwrap();
}
