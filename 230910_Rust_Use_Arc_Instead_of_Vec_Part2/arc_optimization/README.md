# Source

https://doc.rust-lang.org/std/sync/struct.Arc.html#method.decrement_strong_count

# Result

```
$ cargo run
[
    Data1 {
        dummy_num: 1,
    },
    Data1 {
        dummy_num: 2,
    },
    Data1 {
        dummy_num: 3,
    },
    Data1 {
        dummy_num: 4,
    },
    Data1 {
        dummy_num: 5,
    },
    Data1 {
        dummy_num: 6,
    },
    Data1 {
        dummy_num: 7,
    },
    Data1 {
        dummy_num: 8,
    },
    Data1 {
        dummy_num: 9,
    },
    Data1 {
        dummy_num: 10,
    },
    Data1 {
        dummy_num: 11,
    },
]

```



```
$ cargo size --release -- -A -x

arc_optimization  :
section                 size          addr
__text               0x359d8   0x1000039b0
__stubs                0x318   0x100039388
__const               0x55d8   0x1000396a0
__gcc_except_tab      0x1298   0x10003ec78
__unwind_info         0x1908   0x10003ff10
__eh_frame            0x67e0   0x100041818
__got                  0x218   0x100048000
__const               0x24a0   0x100048218
__data                   0x8   0x10004c000
__thread_vars           0xc0   0x10004c008
__thread_data           0x40   0x10004c0c8
__thread_bss            0x30   0x10004c108
__common                0x30   0x10004c138
__bss                   0xd8   0x10004c168
Total                0x46f40

```
