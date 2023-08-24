use std::sync::Mutex;
use std::sync::OnceLock;

static COUNT: OnceLock<Mutex<i32>> = OnceLock::new();

fn function_with_static_variable() {
    let count = COUNT.get_or_init(|| Mutex::new(0));
    let mut count = count.lock().unwrap();
    *count += 1;
    println!("{}", *count);
}
fn main() {
    function_with_static_variable();
    function_with_static_variable();
    function_with_static_variable();
    function_with_static_variable();
}
