fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];

    let mut my_vec_iter = my_vec.iter();

    while let Some(val) = my_vec_iter.next() {
        println!("front poped : {}", val);
    }
    println!("front pop pattern ~");
}
