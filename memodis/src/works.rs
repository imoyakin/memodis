use super::msg;
use super::cmd;
use std::thread;
use crossbeam::crossbeam_channel::{Receiver, RecvError};

pub fn works_start(thread_limit: i32) {
    thread::spawn(move || {
        master(thread_limit);
    });
    for thread_num in 0..thread_limit {
        thread::spawn(move || works(thread_num));
    }
}

fn master(thread_limit:i32) {
    let mut threadctl = Vec::new();
    {
        for i in 0..thread_limit {
            threadctl.push( msg::message_channel.lock().unwrap().get(&i).unwrap().0.clone());
        }
    }

    loop {
        for num in &threadctl {
            match msg::order_channel.1.recv() {
                Err(RecvError) => panic!("fuckyou{}", RecvError),
                Ok(r)=> {
                    let _ = num.send(r);
                },
            }
        }
    }
}

fn works(thread_num: i32) {
    let rx: Receiver<String>;
    {
        let temp = msg::message_channel.lock().unwrap();
        let channel = temp.get(&thread_num);
        match channel {
            None => panic!("can not get message_channel!"),
            Some((_sender, receiver)) => rx = receiver.clone(),
        }
    }
    loop {
        match rx.recv() {
            Err(r) => panic!("what happen?{}",r),
            Ok(r) => {
                let command = r.split(" ").collect::<Vec<_>>();
                // let order = iter.next().expect("first partation must exists");
                let v = cmd::command.get::<str>(&command[0]);
                match v {
                    Some(s) => s.run(&command),
                    None => println!("err"),
                }
            }
        }
    }
}
// fn works(thread_num: i32) {
//     let rx: Receiver<String>;
//     {
//         let ss = msg::message_channel.lock().unwrap();
//         let channel = ss.get(&thread_num);
//         //let channel = msg::message_channel.lock().unwrap().get(&thread_num);
//         match channel {
//             None => panic!("can not get message_channel!"),
//             Some((_sender, receiver)) => rx = receiver.clone(),
//             // Some((_Sender, Receiver)) => {
//             //     while let Ok(r) = Receiver.recv() {
//             //         println!("{}", r);
//             //     }
//             // }
//         }
//     }
//     while let Ok(r) = rx.recv() {
//         println!("{}", r);
//     }
// }
