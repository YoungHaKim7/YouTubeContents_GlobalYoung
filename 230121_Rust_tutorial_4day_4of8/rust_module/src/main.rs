mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_something2() {
        println!("In the bar module");
    }
}

fn main() {
    foo::do_something();
    bar::do_something2();
}
