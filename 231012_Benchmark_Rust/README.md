# Result

```bash

$ hyperfine --warmup 3 './fibonacci_recursive/target/release/fibonacci_recursive' './fibonacci_iterator/target/release/fibonacci_iterator'

Benchmark 1: ./fibonacci_recursive/target/release/fibonacci_recursive
  Time (mean ± σ):     23.930 s ±  0.041 s    [User: 23.847 s, System: 0.073 s]
  Range (min … max):   23.900 s … 24.040 s    10 runs

Benchmark 2: ./fibonacci_iterator/target/release/fibonacci_iterator
  Time (mean ± σ):     16.212 s ±  0.022 s    [User: 16.159 s, System: 0.045 s]
  Range (min … max):   16.175 s … 16.258 s    10 runs

Summary
  ./fibonacci_iterator/target/release/fibonacci_iterator ran
    1.48 ± 0.00 times faster than ./fibonacci_recursive/target/release/fibonacci_recursive

```

# Benchmark Tools

- https://github.com/sharkdp/hyperfine



# Why is iterator so much faster? 

https://www.reddit.com/r/rust/comments/eiwhkn/why_is_iterator_so_much_faster/
