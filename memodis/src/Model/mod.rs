use chashmap;

mod client;
mod dict;
mod obj;

pub mod db;
pub mod msg;

lazy_static! {
    pub static ref DB_LIST: chashmap::CHashMap<i32, db::MemodisDB> = {
       chashmap::CHashMap::new()
    };
}