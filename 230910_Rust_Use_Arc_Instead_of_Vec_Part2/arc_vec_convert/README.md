# Source

- Is it possible to convert a &[T] or Vec<T> to Arc<Mutex<[T]>>? 
  - https://stackoverflow.com/questions/76535332/is-it-possible-to-convert-a-t-or-vect-to-arcmutext

# Result

```
cargo run
   Compiling arc_vec_convert v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230910_Rust_Use_Arc_Instead_of_Vec_Part2/arc_vec_convert)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/arc_vec_convert`
m1: Mutex { data: [1, 2, 3], poisoned: false, .. }
v: [1, 2, 3]
m2: Mutex { data: [1, 2, 3], poisoned: false, .. }
m3: Mutex { data: [6, 7, 8, 9], poisoned: false, .. }

```
