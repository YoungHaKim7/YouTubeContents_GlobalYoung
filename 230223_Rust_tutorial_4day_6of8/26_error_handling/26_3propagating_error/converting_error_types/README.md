# Result

```
$ cargo run
   Compiling converting_error_types v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230223_Rust_tutorial_4day_6of8/26_error_handling/26_3propagating_error/converting_error_types)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/converting_error_types`

username or error : Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })


$ cargo run
   Compiling converting_error_types v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230223_Rust_tutorial_4day_6of8/26_error_handling/26_3propagating_error/converting_error_types)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/converting_error_types`
username or error : Ok("alice")
```
