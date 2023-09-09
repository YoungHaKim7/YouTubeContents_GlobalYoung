# Result

```
$ cargo size --release -- -A -x

a02_arc_array_test  :
section                 size          addr
__text               0x35204   0x100004594
__stubs                0x318   0x100039798
__const               0x54f8   0x100039ab0
__gcc_except_tab      0x1254   0x10003efa8
__unwind_info         0x17e4   0x1000401fc
__eh_frame            0x6618   0x1000419e0
__got                  0x218   0x100048000
__const               0x23f8   0x100048218
__data                   0x8   0x10004c000
__thread_vars           0xc0   0x10004c008
__thread_data           0x40   0x10004c0c8
__thread_bss            0x30   0x10004c108
__common                0x30   0x10004c138
__bss                   0xf8   0x10004c168
Total                0x462d4
```

```
$ cargo size --release -- -A -x

   Compiling a02_arc_array_test v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230909_Rust_Use_Arc_Instead_of_Vec/a02_arc_array_test)
    Finished release [optimized] target(s) in 0.12s
warning: field `dummy_num` is never read
 --> src/main.rs:5:5
  |
4 | struct Data1 {
  |        ----- field in this struct
5 |     dummy_num: u32,
  |     ^^^^^^^^^
  |
  = note: `Data1` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

a02_arc_array_test  :
section                 size          addr
__text               0x35204   0x100004594
__stubs                0x318   0x100039798
__const               0x54f8   0x100039ab0
__gcc_except_tab      0x1254   0x10003efa8
__unwind_info         0x17e4   0x1000401fc
__eh_frame            0x6618   0x1000419e0
__got                  0x218   0x100048000
__const               0x23f8   0x100048218
__data                   0x8   0x10004c000
__thread_vars           0xc0   0x10004c008
__thread_data           0x40   0x10004c0c8
__thread_bss            0x30   0x10004c108
__common                0x30   0x10004c138
__bss                   0xf8   0x10004c168
Total                0x462d4

```

https://github.com/rust-embedded/cargo-binutils
