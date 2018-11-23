
use crossbeam::crossbeam_channel::{bounded,Sender,Receiver};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref order_channel : (Sender<String>, Receiver<String>) = bounded(0);

    pub static ref message_channel : Mutex<HashMap<i32, (Sender<String>, Receiver<String>)>> = {
        Mutex::new(HashMap::new())
    };
}

pub fn inital_message_channel(n:i32) {
    for i in 0..n {
        message_channel.lock().unwrap().insert(i, bounded(0));
    }
}