use std::fmt::Display;

fn print<T: Display>(x: T) {
    println!("your value : {x}");
}

fn main() {
    print(123);
    print("Hello");
}
