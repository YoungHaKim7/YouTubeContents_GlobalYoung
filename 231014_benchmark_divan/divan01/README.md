# Result

```bash

$ cargo bench

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\example.rs (target\release\deps\example-9abdf2c7640556bb.exe)
Pinned thread to core 0
example                         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ fibonacci                    1.324 µs      │ 7.024 µs      │ 1.337 µs      │ 1.597 µs      │ 100     │ 800
├─ fibonacci_fast               0.095 ns      │ 0.139 ns      │ 0.096 ns      │ 0.097 ns      │ 100     │ 6553600
├─ fibonacci_iterator           0.095 ns      │ 0.56 ns       │ 0.096 ns      │ 0.107 ns      │ 100     │ 6553600
├─ fibonacci_iterator_compute   0.488 ns      │ 1.3 ns        │ 0.492 ns      │ 0.534 ns      │ 100     │ 3276800
├─ fibonacci_recursive          0.095 ns      │ 0.098 ns      │ 0.096 ns      │ 0.096 ns      │ 100     │ 6553600
╰─ fibonacci_recursive_compute  0.488 ns      │ 2.509 ns      │ 0.488 ns      │ 0.568 ns      │ 100     │ 3276800

```
