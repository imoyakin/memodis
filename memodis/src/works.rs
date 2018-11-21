use std::thread;
use super::msg;

pub fn works_start(thread_num:i32) {
    thread::spawn(move || {
        master();
    });
    for _ in 0..thread_num {
        thread::spawn(move || {
            works()
        });
    }
}

fn master() {
    while let Ok(r) = msg::order_channel.1.recv() {
        let command_split = r.split(" ");
        for s in command_split{
            println!("{}", s)
        }
    }
}

fn works() {
   
}