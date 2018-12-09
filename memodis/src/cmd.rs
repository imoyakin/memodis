use std::collections::HashMap;

pub struct MemodisCommand {
    pub commamd_proc: fn(data: &Vec<&str>),
    pub arity: i32,
    pub flag: i32,
    pub microseconds: f64,
}

impl MemodisCommand {
    fn new(commamd_proc:fn(data: &Vec<&str>)) -> MemodisCommand {
        let s = MemodisCommand {
            commamd_proc:commamd_proc,
            arity:1,
            flag:1,
            microseconds:1f64,
        };
        s
    }

    pub fn run(&self, data: &Vec<&str>) {
        (self.commamd_proc)(data);
    }
}

lazy_static! {
    pub static ref command: HashMap<&'static str, MemodisCommand> = {
        let mut cmap = HashMap::new();
        cmap.insert("GET", MemodisCommand::new(test));
        cmap.insert("SET", MemodisCommand::new(test));
        cmap
    };
}

fn test(data: &Vec<&str>) {
    for i in data {
        println!("{:?}", i);
    }
}