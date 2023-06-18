use derive_macros_in_rust01::Reflective;

struct Foo {
    a: i32,
    b: bool,
    c: String,
}

impl Reflective for Foo {
    fn name(&self) -> &'static str {
        "Foo"
    }
}

fn main() {
    let foo = Foo {
        a: 4,
        b: false,
        c: "foo".to_string(),
    };

    println!("The name of struct : {}", foo.name());
}
