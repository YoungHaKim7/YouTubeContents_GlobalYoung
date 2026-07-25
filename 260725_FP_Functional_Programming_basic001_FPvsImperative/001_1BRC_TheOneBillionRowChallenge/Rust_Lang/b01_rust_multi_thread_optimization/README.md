# Result


```bash


```

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

