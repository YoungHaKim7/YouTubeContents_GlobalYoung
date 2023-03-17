fn returnt<T>(foo: T) -> T {
    foo
}

fn printme<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}

fn main() {}
