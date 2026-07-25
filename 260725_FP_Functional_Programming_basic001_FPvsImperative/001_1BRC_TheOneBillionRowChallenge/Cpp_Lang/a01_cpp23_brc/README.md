# AI

- I've converted the Rust code to C++23. Here are the key changes:

## Key Conversions:

**Rust → C++23 equivalents:**
- `rayon::prelude::*` → `<thread>` with manual thread management
- `HashMap<K, V>` → `std::unordered_map<K, V>`
- `Option<T>` → `std::pair<std::string, T>` (empty string = None)
- `PathBuf` → `std::filesystem::path`
- `.expect()` → try-catch exception handling
- `format!` macro → `std::format` (C++20/23)

**C++23 features used:**
- `std::format` for type-safe formatting
- Structured bindings: `auto [name, stats]`
- `std::filesystem::path` for cross-platform paths
- Range-based for loops with initializer

**Parallel processing:**
- Instead of Rayon's `into_par_iter()`, used `std::thread` with manual chunking
- Each thread processes a chunk independently
- Results merged in a thread-safe manner (each thread writes to its own result map)

**Compilation command:**
```bash
g++ -std=c++23 -O3 -pthread src/main.cpp -o brc
```

The C++ version maintains the same architecture and performance characteristics as the Rust original.
