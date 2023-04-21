# Result

```

$ cargo run

   Compiling error_rwlock01 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/html_la/YouTubeContents_GlobalYoung/999996_Understanding_Rust_Thread_Safety/error_rwlock01)
error[E0277]: `Sender<u8>` cannot be shared between threads safely
   --> src/main.rs:8:24
    |
8   |       std::thread::spawn(move || {
    |  _____------------------_^
    | |     |
    | |     required by a bound introduced by this call
9   | |         x.read().unwrap().send(4u8).unwrap();
10  | |     });
    | |_____^ `Sender<u8>` cannot be shared between threads safely
    |
    = help: the trait `Sync` is not implemented for `Sender<u8>`
    = note: required for `RwLock<Sender<u8>>` to implement `Sync`
    = note: required for `Arc<RwLock<Sender<u8>>>` to implement `Send`
note: required because it's used within this closure
   --> src/main.rs:8:24
    |
8   |     std::thread::spawn(move || {
    |                        ^^^^^^^
note: required by a bound in `spawn`
   --> /Users/globalyoung/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:714:8
    |
714 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error_rwlock01` due to previous error

```
