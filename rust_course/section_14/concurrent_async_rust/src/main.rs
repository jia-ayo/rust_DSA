// use std::sync::mpsc;
// use std::thread;
use std::sync::{Arc, Mutex};
// use std::rc::Rc;
use num::{one, Biguint};
use rayon::prelude::*;
use std::time::Instant;

fn factorial(num: u32) -> Biguint {
    if num == 0 || num == 1 {
        return Biguint::one();
    } else {
        (1..=num)
            .map(Biguint::from)
            .reduce(|acc, x| acc * x)
            .unwrap();
    }
}

fn main() {
    println!("{}", factorial(3));
    // let handle = thread::spawn(move || {
    //     println!("Hello, world!");
    // });
    // handle.join().unwrap();
    // println!("Hello from main");

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || println!("{:?}", v));

    // let mut thread_handles = Vec::new();

    // for e in v {
    //     thread_handles.push(thread::spawn(move || println!("thread {:?}", e)))
    // }
    // println!("Main thread");

    // for handle in thread_handles{
    //     handle.join().unwrap();
    // }

    // let (transmitter, reciever) = mpsc::sync_channel(1000);
    // let tx = transmitter.clone();

    // let val = String::from("Transmitting");
    // thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // let msg = reciever.recv().unwrap();
    // println!("{:?}", msg);

    // thread::spawn(move || {
    //     let vec = vec![
    //         String::from("Transmitting"),
    //         String::from("from"),
    //         String::from("Original"),
    //     ];
    //     for val in vec {
    //         transmitter.send(val).unwrap();
    //     }
    // });

    // thread::spawn(move || {
    //     let vec = vec![
    //         String::from("Clone"),
    //         String::from("is"),
    //         String::from("Transmitting"),
    //     ];
    //     for val in vec {
    //         tx.send(val).unwrap();
    //     }
    // });

    // for rec in reciever {
    //     println!("{}", rec);
    // }

    // let rc1 = Arc::new(String::from("Test"));
    // let rc2 = rc1.clone();

    // thread::spawn(move||{
    //     rc2;
    // });

    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..8 {
    //     let counter = Arc::clone(&counter);
    //     let handle = std::thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         // let mut num2 = counter.lock().unwrap(); //deadlock if you do this
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("{}", counter.lock().unwrap())
}
