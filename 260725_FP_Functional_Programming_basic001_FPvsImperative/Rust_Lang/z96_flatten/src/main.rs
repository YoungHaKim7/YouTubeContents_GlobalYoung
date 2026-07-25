fn main() {
    let items = vec![Some(1), None, Some(3)];

    // Idiomatic way to safely unwrap and loop through only matching data
    for item in items.into_iter().flatten() {
        println!("Found valid number: {}", item);
    }
}
