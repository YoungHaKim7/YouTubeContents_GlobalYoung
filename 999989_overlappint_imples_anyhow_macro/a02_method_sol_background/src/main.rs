struct Value;

trait Print {
    fn print(self);
}

impl Print for Value {
    fn print(self) {
        println!("called on Value");
    }
}

impl Print for &Value {
    fn print(self) {
        println!("called on &Value");
    }
}

fn main() {
    let v = Value;
    v.print();
}
