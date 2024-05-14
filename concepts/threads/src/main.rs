use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Threads
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Print from spawned thread {i}");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // handle.join().unwrap();
    //
    // for i in 1..5 {
    //     println!("Print from main thread {i}");
    // }

    // Channels
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread")
        ];

        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("other")
        ];

        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Received: {received}");
    }
}
