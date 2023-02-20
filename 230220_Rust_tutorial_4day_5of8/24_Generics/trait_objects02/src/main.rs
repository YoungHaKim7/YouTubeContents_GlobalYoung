use std::fmt::Display;

// fn print<T: Display>(x: T) {
    // println!("your value : {x}");
// }

fn main() {
    // print(123);
    // print("Hello");
    let xs: Vec<Box<dyn Display>> = vec![Box::new(123), Box::new("Hello")];
    for x in xs {
        println!("x: {x}");
    }
}
