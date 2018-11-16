extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Config {
    pub thread_limit: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Conf {
    pub config: Option<Vec<Config>>,
}

pub fn read_config() {
    let file_path = "Config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) =>f,
        Err(e) => panic!("no such file {} exception {}", file_path, e)
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file:{}", e)
    };

    println!("{:?}", str_val);

    let config: Conf = toml::from_str(&str_val).unwrap();

    for x in config.config.unwrap() {
        println!("{:?}", x);
    }
}