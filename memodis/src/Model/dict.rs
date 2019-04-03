use std::collections::HashMap;
use std::any::Any;
use std::fmt::Debug;

use super::obj;

enum DictObject {
    STR(obj::MemodisObject<String>),
    I32(obj::MemodisObject<i32>),
    None,
}

///Dict
/// # Dict:key space
pub struct Dict {
    dictht:HashMap<String, DictObject>,
    // dictEntry **table;
    // dictType *type;
    // unsigned long size;
    // unsigned long sizemask;
    // unsigned long used;
    // void *privdata;
}

// typedef struct dictEntry {
//     void *key;
//     void *val;
//     struct dictEntry *next;
// } dictEntry;

// typedef struct dictType {
//     unsigned int (*hashFunction)(const void *key);
//     void *(*keyDup)(void *privdata, const void *key);
//     void *(*valDup)(void *privdata, const void *obj);
//     int (*keyCompare)(void *privdata, const void *key1, const void *key2);
//     void (*keyDestructor)(void *privdata, void *key);
//     void (*valDestructor)(void *privdata, void *obj);
// } dictType;

impl Dict {
    //new 
    pub fn new() -> Dict {
        Dict {
            dictht:HashMap::new(),
        }
    }

    pub fn destroy() {
        
    }
}

trait CURD {
    fn Create<T:Any + Debug>(&mut self, key:String, value:&T)->bool;
    // fn Update(&self, value:T)->bool;
    // fn Retrieve();
    fn Delete(&self, key:String)->bool;
}

impl CURD for Dict{
    fn Create<T:Any+Debug>(&mut self, key: String, value: &T)->bool {
        let value = value as &Any;
        if let Some(string) = value.downcast_ref::<String>() {
            let v = DictObject::STR(obj::MemodisObject::new(string.clone()));
            self.dictht.insert(key, v);
            return true
        };
        if let Some(i32) = value.downcast_ref::<i32>() {
            let v = DictObject::I32(obj::MemodisObject::new(*i32));
            self.dictht.insert(key, v);
            return true
        };
        false
    }

    fn Delete(&self, key: String)->bool {
        //todo
        true 
    }
}