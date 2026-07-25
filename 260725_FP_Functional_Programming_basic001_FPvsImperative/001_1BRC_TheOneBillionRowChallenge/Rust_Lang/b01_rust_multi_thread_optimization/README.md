# Result


```bash


```

# clippy 수정 후

```bash
________________________________________________________
Executed in   44.90 millis    fish           external
   usr time   48.08 millis    0.00 micros   48.08 millis
   sys time   12.93 millis  670.00 micros   12.26 millis

   
```

# time분석

- Looking at this timing output from your fish shell execution, here's the analysis:

## Time Breakdown

**Wall-clock (real) time: 44.90 ms**
- The actual elapsed time from start to finish

**User CPU time: 48.08 ms**
- Time spent in user-mode (non-kernel) execution
- Slightly MORE than wall-clock time

**System CPU time: 12.93 ms**
- Time spent in kernel-mode execution
- Total: 6070.00 microseconds shown in middle column

## Key Observations

🔍 **Multi-threading indicator**: The fact that total CPU time (48.08 + 12.93 = 60.01 ms) exceeds wall-clock time (44.90 ms) strongly suggests this command used **multiple threads**. CPU time accumulates across all threads, so if you have 2+ cores working in parallel, total CPU time can exceed real time.

🔍 **CPU efficiency**: 
- User: 48.08 ms / 44.90 ms = ~107% (suggests ~2 threads or modest parallelism)
- System: ~12.93 ms is about 21.5% of total CPU time (reasonable overhead)

🔍 **Performance context**: For a Rust program (given your project path), this is quite fast! Sub-50ms execution time is excellent for most command-line tools.

## What This Tells Us

Your Rust multi-threaded optimization project appears to be working well - the timing profile shows effective parallel execution with minimal system overhead.

# fix

- Perfect! Both Clippy warnings have been fixed. Here's what I changed:

1. **Line 89**: Changed `filter_map(|line| line.ok())` to `map_while(Result::ok)`
   - This prevents potential infinite loops if the iterator repeatedly produces `Err` values
   - `map_while` stops iteration on the first `Err`, while `filter_map` would continue forever

2. **Line 95**: Changed manual division ceiling calculation to use the built-in `div_ceil()` method
   - Before: `(lines.len() + num_threads - 1) / num_threads`
   - After: `lines.len().div_ceil(num_threads)`
   - More idiomatic and clearer intent

The code now passes Clippy checks with no warnings!

