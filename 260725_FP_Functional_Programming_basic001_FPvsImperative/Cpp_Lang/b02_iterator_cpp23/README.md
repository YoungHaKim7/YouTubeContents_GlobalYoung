# Result

```bash
=== Functional Iterator Demo ===

1. Custom Functional Iterator:
  Iterator value: 1
  Iterator value: 2
  Iterator value: 3
  Iterator value: 4
  Iterator value: 5

2. Functional pipeline with error handling:
  Pipeline(36) -> 1
  Pipeline(0) -> 0
  Pipeline(-9) -> Error: Negative square root
  Pipeline(100) -> 1.6666666666666667

3. Statistics calculation:
  Dataset size: 5 -> Mean: 3.00, Sum: 15.00
  Dataset size: 0 -> Error: Empty input
  Dataset size: 3 -> Mean: 20.00, Sum: 60.00

4. Functional filter and transform:
  Even numbers squared:
    4
    16
    36
    64
    100

5. Functional composition:
  Numbers > 5, doubled, first 3:
    12
    14
    16

6. Iterator algorithms:
  Original:
    3
    1
    4
    1
    5
    9
    2
    6
  Sorted:
    1
    1
    2
    3
    4
    5
    6
    9
  Found 5 at position: 5

✓ All functional iterator examples completed successfully!
```

Perfect! The functional iterator implementation is now working successfully. Here's what I've created:

## Key Features Implemented:

### 1. **Custom Functional Iterator**
- `FunctionalIterator<T>` class implementing standard iterator concepts
- `FunctionalRange<T>` for generating ranges using the custom iterator
- `make_range()` helper function for creating ranges

### 2. **Functional Pipeline with Error Handling**
- Uses `std::expected<T, E>` for monadic-style error handling (C++23)
- Chain: `sqrt(input) / 2 / 3` with proper error propagation
- Handles division by zero and negative square roots

### 3. **Statistics Calculator**
- Functional style using algorithms
- Returns `std::expected` for error cases

### 4. **Functional Filter & Transform**
- Template function combining filter and transform operations
- Uses C++23 ranges and views

### 5. **Functional Composition**
- Demonstrates chaining multiple range adaptors
- Filter → Transform → Take pattern

### 6. **Iterator Algorithms**
- `std::ranges::sort` and `std::ranges::find`
- `std::distance` for iterator arithmetic

The implementation compiles and runs correctly with C++23 (`-std=c++2c`). All functional iterator examples are working as expected, showing custom iterator implementations alongside modern C++23 functional programming features.
