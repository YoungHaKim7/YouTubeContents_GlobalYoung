use std::fmt::{Display, Write};

pub trait DisplayToString {
    fn my_to_string(&self) -> String;
}

// General impl that applies to any T with a Display impl.
//
// Note that the Self type of this impl is &T and so the method argument
// is actually &&T! That makes this impl lower priority during method
// resolution if the impl that accepts &String would also apply.
impl<T: Display> DisplayToString for &T {
    fn my_to_string(&self) -> String {
        println!("called blanket impl");

        let mut buf = String::new();
        buf.write_fmt(format_args!("{}", self)).unwrap();
        buf.shrink_to_fit();
        buf
    }
}

pub trait StringToString {
    fn my_to_string(&self) -> String;
}

// Specialized impl to bypass the relatively expensive std::fmt machinery.
//
// The method argument is typed &String.
impl StringToString for String {
    fn my_to_string(&self) -> String {
        println!("called specialized impl");

        self.clone()
    }
}

macro_rules! convert_to_strings {
    ($($e:expr),*) => {
        [$(
            (&$e).my_to_string()
        ),*]
    };
}

fn main() {
    let owned_string = "hacks".to_owned();
    let strings = convert_to_strings![1, "&str", owned_string];
    println!("{:?}", strings);
}
