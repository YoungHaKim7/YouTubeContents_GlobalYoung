# I'll create a demonstration that shows:
  - `std::optional` for nullable values
  - `std::expected` for operations that can fail with specific error types
  - Functional error handling with monadic-style chaining

# Result

```bash
./target/debug/a05_error_handling
=== Functional Error Handling Demo ===

1. std::optional (nullable values):
  '42' -> 84
  '100' -> 200
  'abc' -> failed (invalid or negative)
  '-50' -> failed (invalid or negative)
  '200' -> 400

2. std::expected (operations with errors):
  process_value(4) -> 10
  process_value(0) -> Error: Division by zero
  process_value(-4) -> Error: Negative square root
  process_value(100) -> 2

3. Statistics calculation:
  Dataset size: 5 -> Average: 3
  Dataset size: 0 -> Error: Empty input
  Dataset size: 3 -> Average: 20

4. Functional pipeline:
  Pipeline(36) -> 1.9999999999999996
  Pipeline(0) -> 0
  Pipeline(-9) -> Error: Negative square root

✓ All error handling examples completed successfully!
```
