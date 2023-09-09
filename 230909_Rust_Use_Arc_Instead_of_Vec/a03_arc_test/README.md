# Result

```
$ cargo run
Arc db : [Person { name: "John Doe", age: 30 }, Person { name: "Gyoung", age: 40 }]


$ cargo size --release -- -A -x
a03_arc_test  :
section                 size          addr
__text               0x35668   0x100003e30
__stubs                0x318   0x100039498
__gcc_except_tab      0x1290   0x1000397b0
__const               0x5568   0x10003aa40
__unwind_info         0x18c0   0x10003ffa8
__eh_frame            0x6798   0x100041868
__got                  0x218   0x100048000
__const               0x2488   0x100048218
__data                   0x8   0x10004c000
__thread_vars           0xc0   0x10004c008
__thread_data           0x40   0x10004c0c8
__thread_bss            0x30   0x10004c108
__common                0x30   0x10004c138
__bss                   0xd8   0x10004c168
Total                0x46ab0

```
