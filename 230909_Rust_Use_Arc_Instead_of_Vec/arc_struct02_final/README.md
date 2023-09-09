# Result

```
$ cargo size
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
warning: unused import: `mem`
 --> src/main.rs:2:5
  |
2 |     mem,
  |     ^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:12:15
   |
12 | fn add_arrays(mut foo: TwoArrays) {
   |               ----^^^
   |               |
   |               help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: unused variable: `foo`
  --> src/main.rs:34:9
   |
34 |     let foo = Arc::new(Mutex::new(&two_arrays));
   |         ^^^ help: if this is intentional, prefix it with an underscore: `_foo`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: 3 warnings emitted

__TEXT	__DATA	__OBJC	others	dec	hex
311296	16384	0	4295229440	4295557120	100090000	



$ cargo run
   Compiling arc_struct02_final v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/999990_Rust_Use_Arc_Instead_of_Vec/arc_struct02_final)
warning: unused import: `mem`
 --> src/main.rs:2:5
  |
2 |     mem,
  |     ^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:12:15
   |
12 | fn add_arrays(mut foo: TwoArrays) {
   |               ----^^^
   |               |
   |               help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: unused variable: `foo`
  --> src/main.rs:34:9
   |
34 |     let foo = Arc::new(Mutex::new(&two_arrays));
   |         ^^^ help: if this is intentional, prefix it with an underscore: `_foo`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `arc_struct02_final` (bin "arc_struct02_final") generated 3 warnings (run `cargo fix --bin "arc_struct02_final"` to apply 3 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.62s
     Running `target/debug/arc_struct02_final`
Arrays A: [1, 2, 3]
Arrays A: [4, 5, 6]

```
