# Result

```
cargo run
   Compiling monomorphization v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230220_Rust_tutorial_4day_5of8/24_Generics/monomorphization)
warning: type `Option_i32` should have an upper camel case name
 --> src/main.rs:2:6
  |
2 | enum Option_i32 {
  |      ^^^^^^^^^^ help: convert the identifier to upper camel case: `OptionI32`
  |
  = note: `#[warn(non_camel_case_types)]` on by default

warning: type `Option_f64` should have an upper camel case name
 --> src/main.rs:7:6
  |
7 | enum Option_f64 {
  |      ^^^^^^^^^^ help: convert the identifier to upper camel case: `OptionF64`

warning: unused variable: `integer`
  --> src/main.rs:13:9
   |
13 |     let integer = Option_i32::Some(5);
   |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_integer`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `float`
  --> src/main.rs:14:9
   |
14 |     let float = Option_f64::Some(5.0);
   |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_float`

warning: variant `None` is never constructed
 --> src/main.rs:4:5
  |
2 | enum Option_i32 {
  |      ---------- variant in this enum
3 |     Some(i32),
4 |     None,
  |     ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: variant `None` is never constructed
 --> src/main.rs:9:5
  |
7 | enum Option_f64 {
  |      ---------- variant in this enum
8 |     Some(f64),
9 |     None,
  |     ^^^^

warning: `monomorphization` (bin "monomorphization") generated 6 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/monomorphization`
```
