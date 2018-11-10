use std::io::Read;
use std::io::Write;
use std::thread;
use std::fs::File;
use std::net::{TcpListener, TcpStream};

fn main() {
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

        println!("{}", String::from_utf8_lossy(&buf[..]));
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
