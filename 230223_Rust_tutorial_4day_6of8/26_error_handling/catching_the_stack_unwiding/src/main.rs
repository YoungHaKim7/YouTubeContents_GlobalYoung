use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        println!("oh no!");
    });

    assert!(result.is_err());
}
