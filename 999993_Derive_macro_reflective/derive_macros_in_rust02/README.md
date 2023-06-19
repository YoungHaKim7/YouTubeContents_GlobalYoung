# Result

```
$ cargo expand --bin derive_macros_in_rust02

    Checking derive_macros_in_rust02 v0.1.0 (D:\young_project\rust_lang\YouTubeContents_GlobalYoung\999993_Derive_macro_reflective\derive_macros_in_rust02)
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use derive_macros_in_rust02::Reflective;
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
    {
        ::std::io::_print(format_args!("The name of struct : {0}\n", foo.name()));
    };
}
```


- cargo expand --lib

```
$ cargo expand --lib
  Checking derive_macros_in_rust02 v0.1.0 (D:\young_project\rust_lang\YouTubeContents_GlobalYoung\999993_Derive_macro_reflective\derive_macros_in_rust02)
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use reflective_derive::Reflective;
pub trait Reflective {
    fn name(&self) -> &'static str;
}   
```