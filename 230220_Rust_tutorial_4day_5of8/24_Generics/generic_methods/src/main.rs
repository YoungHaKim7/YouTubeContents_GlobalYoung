#[derive(Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.0
    }
}

fn main() {
    let p = Point(5, 10);
    println!("p.x = {}", p.x());
}
