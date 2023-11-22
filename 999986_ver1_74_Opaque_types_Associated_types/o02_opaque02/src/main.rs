#![feature(type_alias_impl_trait)]
trait MyTrait {}

type Foo<T> = impl MyTrait;

struct MyStruct<A, B, C> {
    a: A,
    b: B,
    c: C,
}
impl<A, B, C> MyTrait for MyStruct<A, B, C> {}

fn foo<T>(t: T) -> Foo<T> {
    // This tells the compiler that `for<T> Foo<T> == MyStruct<i32, T, &'static str>`
    MyStruct {
        a: 1i32,
        b: t,
        c: "",
    }
}

fn main() {
    println!("Hello, world!");
}
