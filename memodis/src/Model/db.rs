use crate::model::dict;

pub struct MemodisDB {
    //数据库键空间，保持存折数据库中的所有键值对
    pub dbdict: dict::Dict,
    //键的过期时间，字典的键为键， 字典的值为过期时间 unix时间戳
    pub expires:  dict::Dict,
    //正处于阻塞状态的键
    pub blocking_keys: dict::Dict,
    //可以解除阻塞的键
    pub read_keys: dict::Dict,
    //正在被watch命令监视的键
    pub watched_keys: dict::Dict,
    //数据库号码
    pub id:i32,
    //数据库的键的平均TTL，统计信息
    pub avg_ttl:i64,
}

impl MemodisDB {
    pub fn new(id:i32) -> MemodisDB {
        MemodisDB{
            dbdict:dict::Dict::new(),
            expires: dict::Dict::new(),
            blocking_keys: dict::Dict::new(),
            read_keys: dict::Dict::new(),
            watched_keys: dict::Dict::new(),
            id,
            avg_ttl:0,
        }
    }

    pub fn destroy() {
        
    }
}