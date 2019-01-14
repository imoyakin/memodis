use super::dict;

///memodisObject  
pub struct MemodisObject<'a,T>{
    //类型
    pub data_type: i32,
    //编码
    pub encoding: i32,
    //独享最后一次被访问的时间
    pub lru:i64,
    //引用计数
    pub refcount:i32,
    //指向实际值的引用
    pub data:&'a T,
}

impl<'a,T>  MemodisObject<'a, T> {
    fn new(data:&'a T) -> MemodisObject<'a,T>{
        MemodisObject{
            data_type:1,
            encoding:1,
            lru:1,
            refcount:1,
            data:data,
        }
    }
}


///memodisDB
/// id database id
pub struct Memodisdb {
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
    //数据库的键的平均TTL，统计信息，
    pub avg_ttl:i64,
}

impl Memodisdb {
    pub fn new(id:i32) -> Memodisdb {
        Memodisdb{
            dbdict:dict::Dict::new(),
            expires: dict::Dict::new(),
            blocking_keys: dict::Dict::new(),
            read_keys: dict::Dict::new(),
            watched_keys: dict::Dict::new(),
            id:id,
            avg_ttl:0,
        }
    }

    pub fn destroy() {
        
    }
}