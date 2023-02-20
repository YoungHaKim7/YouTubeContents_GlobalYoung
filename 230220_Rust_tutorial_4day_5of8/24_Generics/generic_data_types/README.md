# Result

```
cargo run
   Compiling generic_data_types v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230220_Rust_tutorial_4day_5of8/24_Generics/generic_data_types)
warning: fields `x` and `y` are never read
 --> src/main.rs:3:5
  |
2 | struct Point<T> {
  |        ----- fields in this struct
3 |     x: T,
  |     ^
4 |     y: T,
  |     ^
  |
  = note: `Point` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `generic_data_types` (bin "generic_data_types") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/generic_data_types`
[Point](2023-02-20_point.md) { x: 5, y: 10 } and Point { x: 1.0, y: 4.0 }
```
