# Result

```

$ cargo run

   Compiling derive_debug_cargo_expand v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/999993_Derive_macro_reflective/derive_debug_cargo_expand)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/derive_debug_cargo_expand`

Foo: 0
Hello, world!
```


# cargo expand

```

$ cargo expand
    Checking derive_debug_cargo_expand v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/999993_Derive_macro_reflective/derive_debug_cargo_expand)
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt;
struct Foo {
    a: i32,
}
impl fmt::Debug for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Foo: {0}", self.a))
    }
}
fn main() {
    let foo = Foo { a: 0 };
    {
        ::std::io::_print(format_args!("{0:?}\n", foo));
    };
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}  

```
