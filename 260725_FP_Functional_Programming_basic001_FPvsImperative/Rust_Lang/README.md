# The C++23 `std::ranges::fold_left` example can be converted to Rust like this:

| C++23                                    | Rust                 |
| ---------------------------------------- | -------------------- |
| `std::views::iota(1, 11)`                | `1..=10`             |
| `std::ranges::fold_left(...)`            | `.fold(...)`         |
| `0`                                      | `0`                  |
| `[](int acc, int x) { return acc + x; }` | `\|acc, x\| acc + x` |
