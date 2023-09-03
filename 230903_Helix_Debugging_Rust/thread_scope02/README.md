# Result

```

cargo run
   Compiling thread_scope02 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230903_Helix_Debugging_Rust/thread_scope02)
warning: unused variable: `handle`
  --> src/main.rs:25:21
   |
25 |                 let handle = thread::spawn(move || {
   |                     ^^^^^^ help: if this is intentional, prefix it with an underscore: `_handle`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `thread_scope02` (bin "thread_scope02") generated 1 warning (run `cargo fix --bin "thread_scope02"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/thread_scope02`
Thread ID: ThreadId(1) Process ID: 19527 / print lock  count = 1
Thread ID: ThreadId(1) Process ID: 19527 / print lock  count = 2
Thread ID: ThreadId(1) Process ID: 19527 / print lock  count = 3
Thread ID: ThreadId(1) Process ID: 19527 / print lock  count = 4
Thread ID: ThreadId(1) Process ID: 19527 / print lock  count = 5
The final counter value is: 5
All threads completed!
Thread ID: ThreadId(2) Process ID: 19527 / print lock  count = 6
Thread ID: ThreadId(7) Process ID: 19527 / print lock  count = 7
Thread ID: ThreadId(8) Process ID: 19527 / print lock  count = 8
Thread ID: ThreadId(9) Process ID: 19527 / print lock  count = 9
Thread ID: ThreadId(10) Process ID: 19527 / print lock  count = 10
Thread ID: %  
```
