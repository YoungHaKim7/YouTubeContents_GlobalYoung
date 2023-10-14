# Result

```bash

$ cargo bench

warning: `divan01` (bench "example") generated 2 warnings (run `cargo fix --bench "example"` to apply 2 suggestions)
    Finished bench [optimized] target(s) in 0.93s
     Running unittests src\lib.rs (target\release\deps\divan01-4a83797fe2be434b.exe)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\example.rs (target\release\deps\example-9abdf2c7640556bb.exe)
Pinned thread to core 0
example                         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ fibonacci                    1.274 µs      │ 2.462 µs      │ 1.287 µs      │ 1.33 µs       │ 100     │ 800
├─ fibonacci_iterator           0.085 ns      │ 0.126 ns      │ 0.086 ns      │ 0.087 ns      │ 100     │ 6553600
├─ fibonacci_iterator_compute   0.478 ns      │ 0.668 ns      │ 0.478 ns      │ 0.484 ns      │ 100     │ 3276800
├─ fibonacci_recursive          0.085 ns      │ 0.129 ns      │ 0.086 ns      │ 0.087 ns      │ 100     │ 6553600
╰─ fibonacci_recursive_compute  0.475 ns      │ 0.61 ns       │ 0.482 ns      │ 0.482 ns      │ 100     │ 1638400
```
