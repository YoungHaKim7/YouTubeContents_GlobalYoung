# Result

```

$ cargo size --release -- -A -x

   Compiling a01_arc_vec_test v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230909_Rust_Use_Arc_Instead_of_Vec/a01_arc_vec_test)
    Finished release [optimized] target(s) in 0.09s

a01_arc_vec_test  :
section                 size          addr
__text               0x35114   0x100004684
__stubs                0x318   0x100039798
__const               0x54f8   0x100039ab0
__gcc_except_tab      0x1254   0x10003efa8
__unwind_info         0x17e4   0x1000401fc
__eh_frame            0x6620   0x1000419e0
__got                  0x218   0x100048000
__const               0x23f8   0x100048218
__data                   0x8   0x10004c000
__thread_vars           0xc0   0x10004c008
__thread_data           0x40   0x10004c0c8
__thread_bss            0x30   0x10004c108
__common                0x30   0x10004c138
__bss                   0xf8   0x10004c168
Total                0x461ec

```

```

$ cargo size

__TEXT	__DATA	__OBJC	others	dec	hex
294912	16384	0	4295213056	4295524352	100088000


$ cargo run

src/main.rs:23] db = [
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 9,
    },
]
```
