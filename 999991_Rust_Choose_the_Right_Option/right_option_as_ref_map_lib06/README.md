# Result

```

$ cargo nextest run

   Compiling right_option_as_ref_map_lib06 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/999991_Rust_Choose_the_Right_Option/right_option_as_ref_map_lib06)
warning: unused variable: `crunch_a`
  --> src/lib.rs:35:13
   |
35 |         let crunch_a = a.as_ref().map(|data| data.crunch());
   |             ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_crunch_a`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `right_option_as_ref_map_lib06` (lib test) generated 1 warning (run `cargo fix --lib -p right_option_as_ref_map_lib06 --tests` to apply 1 suggestion)
    Finished test [unoptimized + debuginfo] target(s) in 0.12s
    Starting 1 tests across 1 binaries
        PASS [   0.004s] right_option_as_ref_map_lib06 tests::it_works
------------
     Summary [   0.004s] 1 tests run: 1 passed, 0 skipped
```
