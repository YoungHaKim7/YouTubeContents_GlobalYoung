fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];

    while let Some(val) = my_vec.pop() {
        println!("poped : {}", val);
    }
    println!("Reversed pattern ~");
}
