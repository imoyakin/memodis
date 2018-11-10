use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:80").unwrap();

    loop {
        let mut buf = [0;10];
        io::stdin().read(&mut buf)
            .expect("Failed to read line");
        //println!("you typed:{}",buf.trim());
        let _ = stream.write(&mut buf);
        let _ = stream.read(&mut [0; 4096]);
    }
}

// use std::io::prelude::*;
// use std::net::TcpStream;

// fn main(){
//     let mut stream = TcpStream::connect("127.0.0.1:80").unwrap();

//     // ignore the Result
//     let _ = stream.write(&[1]);
//     let _ = stream.read(&mut [0; 128]); // ignore here too
// } // the stream is closed here