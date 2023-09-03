use std::{process, rc::Rc, sync::Mutex, thread};

fn main() {
    let counter = Rc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        println!(
            "Thread ID: {:?} Process ID: {}",
            thread::current().id(),
            process::id(),
        );
        // let handle = thread::spawn(move || {
        //     let mut lock = counter.lock().unwrap();
        //     *lock += 1;
        //     println!(
        //         "Thread ID: {:?} Process ID: {} / print lock  count = {}",
        //         thread::current().id(),
        //         process::id(),
        //         lock
        //     );
        // });
        // handles.push(handle);
    }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    println!("The final counter value is: {}", *counter.lock().unwrap());
}
