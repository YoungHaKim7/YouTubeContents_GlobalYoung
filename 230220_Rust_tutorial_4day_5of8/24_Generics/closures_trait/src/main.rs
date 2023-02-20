fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn main() {
    let add_3 = |x| x + 3;
    let add_5 = |x| x + 5;

    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("add_5: {}", apply_with_log(add_5, 20));
}
