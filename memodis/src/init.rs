use crate::Model::db;

pub fn init() {
    //read file

    //initail database
    let db = db::MemodisDB::new(1);
    
}