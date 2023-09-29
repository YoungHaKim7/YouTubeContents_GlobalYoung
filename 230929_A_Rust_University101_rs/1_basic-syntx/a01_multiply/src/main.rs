fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("10 * 20= {}", multiply(10, 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_mutiplies() {
        assert!(multiply(10, 10) == 100);
    }
}
