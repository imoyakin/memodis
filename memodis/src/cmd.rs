use std::collections::HashMap;

pub struct MemodisCommand {
    pub commamd_proc: fn(),
    pub arity: i32,
    pub flag: i32,
    pub microseconds: f64,
}

impl MemodisCommand {
    fn new(commamd_proc:fn()) -> MemodisCommand {
        let s = MemodisCommand {
            commamd_proc:commamd_proc,
            arity:1,
            flag:1,
            microseconds:1f64,
        };
        s
    }

    pub fn run(&self) {
        (self.commamd_proc)();
    }
}

lazy_static! {
    pub static ref command: HashMap<String, MemodisCommand> = {
        let mut cmap = HashMap::new();
        cmap.insert("GET".to_string(), MemodisCommand::new(test));
        cmap.insert("SET".to_string(), MemodisCommand::new(test));
        cmap
    };
}

fn test() {
    println!("test");
}