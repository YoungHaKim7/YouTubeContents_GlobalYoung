#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let point = Point { x: 42, y: 2.1 };

    dbg!(point);
}
