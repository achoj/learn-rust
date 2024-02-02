// Do not communicate by sharing memory;
// instead, sharing memory by communicating.

// mutex => mutual exclusion.

use std::sync::{/*mpsc,*/ Mutex, Arc};
use std::thread;
// use std::time::Duration;
// use std::rc::Rc;

fn main() {
    // transmitter & receiver.
    /* 
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    
    // // 阻塞
    // // let received = rx.recv().unwrap();
    // thread::sleep(Duration::from_millis(1));

    // // 非阻塞
    // let received  = rx.try_recv().unwrap();
    // println!("Got: {}", received);

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("from"),
            String::from("tx1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx {
        println!("Got: {}", received);
    }
    */

    // Atomic reference counter
    let counter = Arc::new(Mutex::new(10));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num *= 3;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
