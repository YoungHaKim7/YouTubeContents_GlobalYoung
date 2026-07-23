fn main() {
    let numbers = 1..=10;

    let sum = numbers.fold(0, |acc, x| acc + x);

    println!("{sum}");
}
