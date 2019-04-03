use crossbeam::crossbeam_channel::{unbounded,Sender,Receiver};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::net::{TcpStream};

type Hashmapchannel<T> = Mutex<HashMap<i32, T>>;

lazy_static! {
    pub static ref ORDER_CHANNEL : (Sender<OrderChannelMsg>, Receiver<OrderChannelMsg>) = unbounded();

    pub static ref MESSAGE_CHANNEL : Hashmapchannel<(Sender<OrderChannelMsg>, Receiver<OrderChannelMsg>)> = {
        Mutex::new(HashMap::new())
    };

    pub static ref RETURN_CHANNEL : Hashmapchannel<(Sender<String>, Receiver<String>)> = {
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
    pub steam:Arc<Mutex<TcpStream>>,
}

impl OrderChannelMsg {
    pub fn new(command:String, addr:Arc<Mutex<TcpStream>>) -> OrderChannelMsg {
        OrderChannelMsg{
            common: command,
            steam : addr,
        }
    }
}

pub struct ReturnChannalMsg {

}

impl ReturnChannalMsg {
    pub fn new() -> ReturnChannalMsg {
        ReturnChannalMsg {

        }
    }
}