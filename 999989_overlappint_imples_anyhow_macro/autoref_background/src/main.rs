struct Value(i32);

impl Value {
    fn print(&self) {
        println!("it worked! {}", self.0);
    }
}

fn main() {
    let v = Value(0);
    v.print();
}
