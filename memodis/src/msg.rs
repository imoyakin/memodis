
use crossbeam::crossbeam_channel::{unbounded,Sender,Receiver};
use std::collections::HashMap;
use std::sync::Mutex;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

lazy_static! {
    pub static ref ORDER_CHANNEL : (Sender<OrderChannelMsg>, Receiver<OrderChannelMsg>) = unbounded();

    pub static ref MESSAGE_CHANNEL : Mutex<HashMap<i32, (Sender<OrderChannelMsg>, Receiver<OrderChannelMsg>)>> = {
        Mutex::new(HashMap::new())
    };

    pub static ref RETURN_CHANNEL : Mutex<HashMap<i32, (Sender<String>, Receiver<String>)>> = {
        Mutex::new(HashMap::new())
    };
}

pub fn inital_message_channel(n:i32) {
    for i in 0..n {
        MESSAGE_CHANNEL.lock().unwrap().insert(i, unbounded());
        RETURN_CHANNEL.lock().unwrap().insert(i, unbounded());
    }
}

#[derive(Clone)]
pub struct OrderChannelMsg {
    pub common:String,
    pub peer_addr:SocketAddr,
}

impl OrderChannelMsg {
    pub fn new(command:String, addr:SocketAddr) -> OrderChannelMsg {
        OrderChannelMsg{
            common: command,
            peer_addr:addr,
        }
    }
}