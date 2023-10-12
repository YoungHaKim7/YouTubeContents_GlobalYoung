fn fibonacci(count: usize) -> u128 {
    let mut a = 0u128;
    let mut b = 1u128;
    { 1..=count }.for_each(|c| {
        let next = a + b;
        if c % 2 == 0 {
            a = next;
        } else {
            b = next;
        }
    });
    if count % 2 == 0 {
        a
    } else {
        b
    }
}

pub fn main() {
    { 0..100_000_000 }
        .for_each(|_| assert_eq!(fibonacci(185), 205697230343233228174223751303346572685));
}
