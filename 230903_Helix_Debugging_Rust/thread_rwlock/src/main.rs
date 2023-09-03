use std::{process, rc::Rc, sync::RwLock, thread};

fn main() {
    let counter = Rc::new(RwLock::new(0));

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let mut lock = counter.write().unwrap();
        *lock += 1;
        println!(
            "Thread ID: {:?} Process ID: {}/ print lock count {:?}",
            thread::current().id(),
            process::id(),
            lock
        );
    }

    println!("The final counter value is {}", *counter.write().unwrap());
}
