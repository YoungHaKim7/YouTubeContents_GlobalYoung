use std::{
    process,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::scope(|_| {
            let mut lock = counter.lock().unwrap();
            *lock += 1;
            println!(
                "Thread ID: {:?} Process ID: {} / print lock  count = {}",
                thread::current().id(),
                process::id(),
                lock
            );
            for _ in 0..5 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut lock = counter.lock().unwrap();
                    *lock += 1;
                    println!(
                        "Thread ID: {:?} Process ID: {} / print lock  count = {}",
                        thread::current().id(),
                        process::id(),
                        lock
                    );
                });
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle
    }

    println!("The final counter value is: {}", *counter.lock().unwrap());

    // all threads within the scope has to be closed
    // for the program to continue
    println!("All threads completed!");
}
