# Result

```
cargo run
   Compiling default_trait v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230220_Rust_tutorial_4day_5of8/23_3_Important_Traits/default_trait)
warning: fields `x`, `y`, and `z` are never read
 --> src/main.rs:3:5
  |
2 | struct Derived {
  |        ------- fields in this struct
3 |     x: u32,
  |     ^
4 |     y: String,
  |     ^
5 |     z: Implemented,
  |     ^
  |
  = note: `Derived` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `default_trait` (bin "default_trait") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/default_trait`
Derived {
    x: 0,
    y: "",
    z: Implemented(
        "John Smith",
    ),
}
Derived {
    x: 0,
    y: "Y is set!",
    z: Implemented(
        "John Smith",
    ),
}
Derived {
    x: 0,
    y: "",
    z: Implemented(
        "John Smith",
    ),
}
```
