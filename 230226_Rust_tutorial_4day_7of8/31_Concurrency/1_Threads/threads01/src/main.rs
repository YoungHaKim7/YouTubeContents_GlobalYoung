use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..1_000_000 {
            println!("Count in thread: {i} ! ");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..500_000 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
    
}
