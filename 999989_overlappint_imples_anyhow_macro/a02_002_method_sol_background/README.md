# Result

```

called blanket impl
called blanket impl
called specialized impl
["1", "&str", "hacks"]

```

# cargo expand

```
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt::{Display, Write};
pub trait DisplayToString {
    fn my_to_string(&self) -> String;
}
impl<T: Display> DisplayToString for &T {
    fn my_to_string(&self) -> String {
        {
            ::std::io::_print(format_args!("called blanket impl\n"));
        };
        let mut buf = String::new();
        buf.write_fmt(format_args!("{0}", self)).unwrap();
        buf.shrink_to_fit();
        buf
    }
}
pub trait StringToString {
    fn my_to_string(&self) -> String;
}
impl StringToString for String {
    fn my_to_string(&self) -> String {
        {
            ::std::io::_print(format_args!("called specialized impl\n"));
        };
        self.clone()
    }
}
fn main() {
    let owned_string = "hacks".to_owned();
    let strings = [
        (&1).my_to_string(),
        (&"&str").my_to_string(),
        (&owned_string).my_to_string(),
    ];
    {
        ::std::io::_print(format_args!("{0:?}\n", strings));
    };
}
  
```
