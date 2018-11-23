use super::msg;
use std::thread;
use crossbeam::crossbeam_channel::{Receiver};

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
            if let Ok(r) = msg::order_channel.1.recv() {
                let _ = num.send(r);
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
            Ok(r) => println!("{} + {}", r, thread_num)
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
