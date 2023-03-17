# Result

```
cargo run
   Compiling trait_impl v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230317_Rust_Monomorphization_vs_Polymorphism/Polymorphism/trait_impl)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/trait_impl`
Lion says GROWL!
Tiger says GROWL!
Bear says GROWL!
```

# objdump (M1 pro MacBook)

```
objdump --disassemble -S -C ./target/debug/trait_impl
```
