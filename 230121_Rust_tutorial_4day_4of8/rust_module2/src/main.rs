use bar::do_something1;
use foo::do_something2;

mod bar;
mod foo;
fn main() {
    do_something1();
    do_something2();
}
