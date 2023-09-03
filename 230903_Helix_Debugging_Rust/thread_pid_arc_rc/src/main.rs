use std::sync::{Arc, Mutex};
use std::{process, thread};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..10 {
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
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The final counter value is: {}", *counter.lock().unwrap());
}
