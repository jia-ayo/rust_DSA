use std::sync::mpsc;
use std::thread;
use 
fn main() {
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

    let rc1 = Arc::new(String::from("Test"));
    let rc2 = rc1.clone();

    thread::spawn(move||{
        rc2;
    });
}
