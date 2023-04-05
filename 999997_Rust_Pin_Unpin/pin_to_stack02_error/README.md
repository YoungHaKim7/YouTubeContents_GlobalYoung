# Result

```

cargo run
   Compiling pin_to_stack02_error v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230405_Rust_Pin_Unpin/pin_to_stack02_error)
error[E0277]: `PhantomPinned` cannot be unpinned
   --> src/main.rs:53:26
    |
53  |     std::mem::swap(test1.get_mut(), test2.get_mut());
    |                          ^^^^^^^ within `Test`, the trait `Unpin` is not implemented for `PhantomPinned`
    |
    = note: consider using `Box::pin`
note: required because it appears within the type `Test`
   --> src/main.rs:5:8
    |
5   | struct Test {
    |        ^^^^
note: required by a bound in `Pin::<&'a mut T>::get_mut`
   --> /Users/globalyoung/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/pin.rs:834:12
    |
834 |         T: Unpin,
    |            ^^^^^ required by this bound in `Pin::<&'a mut T>::get_mut`

error[E0277]: `PhantomPinned` cannot be unpinned
   --> src/main.rs:53:43
    |
53  |     std::mem::swap(test1.get_mut(), test2.get_mut());
    |                                           ^^^^^^^ within `Test`, the trait `Unpin` is not implemented for `PhantomPinned`
    |
    = note: consider using `Box::pin`
note: required because it appears within the type `Test`
   --> src/main.rs:5:8
    |
5   | struct Test {
    |        ^^^^
note: required by a bound in `Pin::<&'a mut T>::get_mut`
   --> /Users/globalyoung/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/pin.rs:834:12
    |
834 |         T: Unpin,
    |            ^^^^^ required by this bound in `Pin::<&'a mut T>::get_mut`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `pin_to_stack02_error` due to 2 previous errors

```
