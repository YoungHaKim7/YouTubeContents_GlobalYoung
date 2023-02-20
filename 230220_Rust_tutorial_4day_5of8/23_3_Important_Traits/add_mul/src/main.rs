#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p1 = Point { x: 100, y: 200 };
}
