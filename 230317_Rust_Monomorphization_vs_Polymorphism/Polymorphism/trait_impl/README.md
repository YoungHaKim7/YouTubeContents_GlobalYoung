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

# cargo asm

```
$ cargo asm
   Compiling trait_impl v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230317_Rust_Monomorphization_vs_Polymorphism/Polymorphism/trait_impl)
    Finished

 release [optimized] target(s) in 0.15s
Try one of those by name or a sequence number
0 "<trait_impl::Bear as trait_impl::Growler>::growl" [46]
1 "<trait_impl::Lion as trait_impl::Growler>::growl" [48]
2 "<trait_impl::Tiger as trait_impl::Growler>::growl" [46]
3 "core::ops::function::FnOnce::call_once{{vtable.shim}}" [26]
4 "core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>" [8]
5 "std::rt::lang_start" [39]
6 "std::rt::lang_start::{{closure}}" [25]
7 "std::sys_common::backtrace::__rust_begin_short_backtrace" [29]
8 "trait_impl::main" [25]
```
