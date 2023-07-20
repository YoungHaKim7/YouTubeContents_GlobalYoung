// nightly only
#![feature(specialization)]

use std::fmt::{Display, Write};

pub trait MyToString {
    fn my_to_string(&self) -> String;
}

impl<T: Display> MyToString for T {
    fn my_to_string(&self) -> String {
        let mut buf = String::new();
        buf.write_fmt(format_args!("{}", self)).unwrap();
        buf.shrink_to_fit();
        buf
    }
}

impl MyToString for String {
    fn my_to_string(&self) -> String {
        self.clone()
    }
}

fn main() {
    println!("Hello, world!");
}
