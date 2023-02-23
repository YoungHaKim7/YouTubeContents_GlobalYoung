# Result

```
$ cargo run

   Compiling converting_error_types2 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230223_Rust_tutorial_4day_6of8/26_error_handling/26_3propagating_error/converting_error_types2)
warning: unused import: `self`
 --> src/main.rs:3:15
  |
3 | use std::fs::{self, File};
  |               ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `converting_error_types2` (bin "converting_error_types2") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/converting_error_types2`

username or error: Err(IoError(Os { code: 2, kind: NotFound, message: "No such file or directory" }))%


$ cargo run
   Compiling converting_error_types2 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230223_Rust_tutorial_4day_6of8/26_error_handling/26_3propagating_error/converting_error_types2)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/converting_error_types2`
username or error: Err(EmptyUsername("config.dat"))%
```
