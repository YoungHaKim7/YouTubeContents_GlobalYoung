# Result


```
db Person { name: "John Doe", age: 30 }
db Person { name: "Gyoung", age: 40 }


$ cargo size
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
warning: fields `name` and `age` are never read
 --> src/main.rs:5:5
  |
4 | struct Person {
  |        ------ fields in this struct
5 |     name: String,
  |     ^^^^
6 |     age: u32,
  |     ^^^
  |
  = note: `Person` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

__TEXT	__DATA	__OBJC	others	dec	hex
294912	16384	0	4295213056	4295524352	100088000

```


- 다른 Result

```
cargo size
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
warning: variable does not need to be mutable
  --> src/main.rs:17:9
   |
17 |     let mut t1 = thread::spawn(move || {
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:21:9
   |
21 |     let mut t2 = thread::spawn(move || {
   |         ----^^
   |         |
   |         help: remove this `mut`

warning: 2 warnings emitted

__TEXT	__DATA	__OBJC	others	dec	hex
344064	16384	0	4295262208	4295622656	1000a0000	
cargo run
   Compiling arc_final01_01 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/999990_Rust_Use_Arc_Instead_of_Vec/arc_final01_01)
warning: variable does not need to be mutable
  --> src/main.rs:17:9
   |
17 |     let mut t1 = thread::spawn(move || {
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:21:9
   |
21 |     let mut t2 = thread::spawn(move || {
   |         ----^^
   |         |
   |         help: remove this `mut`

warning: `arc_final01_01` (bin "arc_final01_01") generated 2 warnings (run `cargo fix --bin "arc_final01_01"` to apply 2 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 2.20s
     Running `target/debug/arc_final01_01`
The person's age is 30
The person's name is John Doe

```
