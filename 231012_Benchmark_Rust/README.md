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

- test 2

```bash
hyperfine --warmup 3 'fd -H .DS_Store' 'find . -type f -name ".DS_Store"'
Benchmark 1: fd -H .DS_Store
  Time (mean ± σ):      48.1 ms ±   0.5 ms    [User: 76.4 ms, System: 207.9 ms]
  Range (min … max):    47.3 ms …  49.5 ms    58 runs

Benchmark 2: find . -type f -name ".DS_Store"
  Time (mean ± σ):     300.3 ms ±   3.8 ms    [User: 19.6 ms, System: 279.8 ms]
  Range (min … max):   297.0 ms … 307.0 ms    10 runs

Summary
  fd -H .DS_Store ran
    6.24 ± 0.10 times faster than find . -type f -name ".DS_Store"
```

- Rust로 만든 find보다 10배 더 빠름 

https://github.com/sharkdp/fd

# Benchmark Tools

- https://github.com/sharkdp/hyperfine



# Why is iterator so much faster? 

https://www.reddit.com/r/rust/comments/eiwhkn/why_is_iterator_so_much_faster/

# When to use iterators vs. loops?  

https://www.reddit.com/r/rust/comments/16nl7j6/when_to_use_iterators_vs_loops/