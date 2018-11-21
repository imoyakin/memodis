
use crossbeam::crossbeam_channel::{unbounded,Sender,Receiver};

lazy_static! {
    pub static ref order_channel : (Sender<String>, Receiver<String>) = unbounded();
}