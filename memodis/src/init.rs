use crate::config::Config;
use crate::model;

pub fn init(config:Config) {
    //read file

    //initail database
    for i in 1..config.database_num {
        model::DB_LIST.insert(i, model::db::MemodisDB::new(i));
        // model::DB_LIST.lock().unwrap().
        //     insert(i, model::db::MemodisDB::new(i));
    }

    //import database database data
}