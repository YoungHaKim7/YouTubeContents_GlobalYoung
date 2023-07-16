# Result

```rust

annot move out of `*a` which is behind a shared reference
    --> src/lib.rs:35:24
     |
35   |         let crunch_a = a.map(|data| data.crunch());
     |                        ^^-------------------------
     |                        | |
     |                        | `*a` moved due to this method call
     |                        help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
     |                        move occurs because `*a` has type `Option<Data>`, which does not implement the `Copy` trait

```
