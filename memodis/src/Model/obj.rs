///memodisObject  
pub struct MemodisObject<T>{
    //类型
    pub data_type: i32,
    //编码
    pub encoding: i32,
    //独享最后一次被访问的时间
    pub lru:i64,
    //引用计数
    pub refcount:i32,
    //指向实际值的引用
    pub data:T,
}

impl<T>  MemodisObject<T> {
    pub fn new(data: T) -> MemodisObject<T>{
        MemodisObject{
            data_type:1,
            encoding:1,
            lru:1,
            refcount:1,
            data,
        }
    }
}