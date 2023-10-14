# Result

```bash

$ cargo bench

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\example.rs (target\release\deps\example-9abdf2c7640556bb.exe)
Pinned thread to core 0
example                         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ fibonacci                    1.274 µs      │ 1.624 µs      │ 1.412 µs      │ 1.401 µs      │ 100     │ 800
├─ fibonacci_fast               0.095 ns      │ 0.467 ns      │ 0.096 ns      │ 0.1 ns        │ 100     │ 6553600
├─ fibonacci_iterator           0.095 ns      │ 0.139 ns      │ 0.096 ns      │ 0.097 ns      │ 100     │ 6553600
├─ fibonacci_iterator_compute   0.485 ns      │ 0.62 ns       │ 0.492 ns      │ 0.493 ns      │ 100     │ 1638400
├─ fibonacci_recursive          0.095 ns      │ 0.138 ns      │ 0.096 ns      │ 0.097 ns      │ 100     │ 6553600
├─ fibonacci_recursive_compute  0.488 ns      │ 0.495 ns      │ 0.488 ns      │ 0.49 ns       │ 100     │ 3276800
╰─ fibonacci_slow_slow          0.095 ns      │ 0.148 ns      │ 0.104 ns      │ 0.102 ns      │ 100     │ 6553600
```
