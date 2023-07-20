# Result

```

$ cargo run

   Compiling a02_003_realistic_application v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/999989_overlappint_imples_anyhow_macro/a02_003_realistic_application)
warning: unused variable: `error`
 --> src/main.rs:8:40
  |
8 |     pub(crate) fn from_fmt<T: Display>(error: T) -> Self {
  |                                        ^^^^^ help: if this is intentional, prefix it with an underscore: `_error`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `a02_003_realistic_application` (bin "a02_003_realistic_application") generated 1 warning (run `cargo fix --bin "a02_003_realistic_application"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/a02_003_realistic_application`

called Error::from_fmt
called Error::from_std_error
```

# Limitations
The way that this technique applies method resolution cannot be described by a trait bound, so for practical purposes you should think of this technique as working in macros only.

That is, we can't do:

```rust
pub fn demo<T: ???>(value: T) -> String {
    (&value).my_to_string()
}
```

