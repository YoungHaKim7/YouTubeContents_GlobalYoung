use once_cell::sync::Lazy;
use std::sync::Mutex;

static COUNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

fn function_with_static_variable() {
    let mut count = COUNT.lock().unwrap();
    *count += 1;
    println!("{}", *count);
}
fn main() {
    function_with_static_variable();
    function_with_static_variable();
    function_with_static_variable();
    function_with_static_variable();
}
