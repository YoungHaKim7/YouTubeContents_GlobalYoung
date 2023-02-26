use std::thread::{self, Scope};

fn main() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        println!("Length: {}", s.len());
    });
}
