# Result

```

cargo run
   Compiling pin_to_stack03_good v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230405_Rust_Pin_Unpin/pin_to_stack03_good)
warning: associated function `a` is never used
  --> src/main.rs:27:8
   |
27 |     fn a(self: Pin<&Self>) -> &str {
   |        ^
   |
   = note: `#[warn(dead_code)]` on by default

warning: associated function `b` is never used
  --> src/main.rs:31:8
   |
31 |     fn b(self: Pin<&Self>) -> &String {
   |        ^

warning: `pin_to_stack03_good` (bin "pin_to_stack03_good") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/pin_to_stack03_good`
test1.b points to "test1": 0x16d712728...
... and now it points nowhere: 0x0

```
