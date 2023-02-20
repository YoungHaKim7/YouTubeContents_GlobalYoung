# Result

```
cargo run
   Compiling drop_traits v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230220_Rust_tutorial_4day_5of8/23_3_Important_Traits/drop_traits)
warning: unused variable: `b`
  --> src/main.rs:13:13
   |
13 |         let b = Droppable { name: "b" };
   |             ^ help: if this is intentional, prefix it with an underscore: `_b`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `c`
  --> src/main.rs:15:17
   |
15 |             let c = Droppable { name: "c" };
   |                 ^ help: if this is intentional, prefix it with an underscore: `_c`

warning: unused variable: `d`
  --> src/main.rs:16:17
   |
16 |             let d = Droppable { name: "d" };
   |                 ^ help: if this is intentional, prefix it with an underscore: `_d`

warning: `drop_traits` (bin "drop_traits") generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/drop_traits`
Exiting block B
Dropping d
Dropping c
Exiting block A
Dropping b
Dropping a
Exiting main
```
