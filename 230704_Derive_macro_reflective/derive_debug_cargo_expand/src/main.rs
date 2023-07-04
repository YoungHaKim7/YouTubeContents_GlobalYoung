use std::fmt;

// #[derive(Debug)]
struct Foo {
    a: i32,
}

impl fmt::Debug for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Foo: {}", self.a)
    }
}

fn main() {
    let foo = Foo { a: 0 };
    println!("{foo:?}");
    println!("Hello, world!");
}
