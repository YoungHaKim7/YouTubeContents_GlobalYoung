use std::thread;

fn main() {
    let s = String::from("Hello");

    thread::spawn(move || {
        println!("Length: {}", s.len());
    });
}
