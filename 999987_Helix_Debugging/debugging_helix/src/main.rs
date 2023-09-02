fn main() {
    let my_text = Box::new("test_text");
    let mut x = 0;
    for xs in x..10 {
        x += 1;
        println!("test : {}", x);
        println!("string : my text {my_text}");
    }
}
