use std::io::Read;
use std::io::Write;
use std::thread;
use std::net::{TcpListener, TcpStream};

mod config;

fn main() {

    config::read_config();

    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
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

        let command = String::from_utf8_lossy(&buf[..]);
        let command_split = command.split(" ");
        for s in command_split {
            println!("{}", s);
        }

        println!("{}", command);
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
