use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{Read,Write};

use crate::model::*;
use crate::model;

pub struct MemodisCommand {
    pub commamd_proc: fn(data: &msg::OrderChannelMsg),
    pub arity: i32,
    pub flag: i32,
    pub microseconds: f64,
}

impl MemodisCommand {
    fn new(commamd_proc:fn(data: &msg::OrderChannelMsg)) -> MemodisCommand {
        MemodisCommand {
            commamd_proc,
            arity:1,
            flag:1,
            microseconds:1f64,
        }
    }

    pub fn run(&self, data: &msg::OrderChannelMsg) {
        (self.commamd_proc)(data);
    }
}

lazy_static! {
    pub static ref COMMAND: HashMap<&'static str, MemodisCommand> = {
        let mut cmap = HashMap::new();
        cmap.insert("GET", MemodisCommand::new(get));
        cmap.insert("SET", MemodisCommand::new(set));
        cmap
    };
}

fn get(data: &msg::OrderChannelMsg){
    let ret = model::DB_LIST.get_mut(&1);
    
    println!("get收到命令！");
    println!("get执行命令！");
    
}

fn set(data: &msg::OrderChannelMsg) {
    //model::DB_LIST.insert_new(k, v)
}