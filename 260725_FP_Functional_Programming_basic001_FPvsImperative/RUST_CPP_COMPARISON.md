# 📊 Detailed Comparison Table: C++23 vs Rust (Imperative & Functional)

## 🔄 Sum 1 to 10 Example

| **Aspect** | **C++23 Imperative** | **Rust Imperative** | **Rust Functional** | **C++23 Functional** | **Key Differences** |
|:---|:---|:---|:---|:---|:---|
| **Code Example** | `int sum = 0;`<br>`for (int i = 1; i <= 10; ++i) {`<br>&emsp;`sum += i;`<br>`}`<br>`std::println("{}", sum);`<br> | `let mut sum = 0;`<br>`for i in 1..=10 {`<br>&emsp;`sum += i;`<br>`}`<br>`println!("{}", sum);` | `let sum = (1..=10)`<br>&emsp;`.fold(0, \|acc, x\| acc + x);`<br>`println!("{}", sum);` | `auto numbers = std::views::iota(1, 11);`<br>`int sum = std::ranges::fold_left(`<br>&emsp;`numbers, 0, std::plus{}`<br>`);`<br>`std::println("{}", sum);` | Rust's `fold` is more concise than C++23's `fold_left` |
| **Control Flow** | Explicit `for` loop with manual iteration control | Explicit `for` loop with manual iteration control | Iterator-based with functional composition | Algorithm-based (`std::ranges::fold_left`) | FP abstracts iteration control in both languages |
| **State Management** | Manual state accumulation in mutable variable `sum` | Manual state accumulation in mutable variable `sum` | Declarative transformation, immutable by default | Declarative transformation with functional style | Rust encourages immutability by default |
| **Range Creation** | N/A (manual loop) | N/A (manual loop) | `1..=10` (inclusive range) | `std::views::iota(1, 11)` | Rust's range syntax is more concise |
| **Code Lines** | 4 lines | 4 lines | 3 lines | 4-5 lines | Rust functional is most concise |
| **Variable Mutability** | Mutable `int sum` required | Mutable `let mut sum` required | Immutable `let sum` (no mut needed) | Immutable `int sum` (FP style) | FP reduces mutability in both languages |

---

## 🎯 Detailed Feature Comparison

| **Aspect** | **C++23 Imperative** | **Rust Imperative** | **Rust Functional** | **C++23 Functional** | **Analysis** |
|:---|:---|:---|:---|:---|:---|
| **Control Flow** | Explicit `for`/`while` loops with manual iteration | Explicit `for` loops with pattern matching | Iterator methods (`map`, `filter`, `fold`) | Algorithm-based (`std::transform`, `std::ranges`) | Rust's iterator chain is more ergonomic than C++23 ranges |
| **State Management** | Manual state in mutable variables (`vector<int> results`) | Manual state in mutable variables (`let mut vec`) | Zero-cost abstractions, immutable by default | Declarative with minimal state | Rust's ownership system enforces immutability |
| **Iteration Style** | `for (auto& item : items)` loop control | `for item in items` loop control | `items.iter().map(\|x\| ...).collect()` abstraction | `items \| views::transform(func)` abstraction | Both support FP, Rust's syntax is cleaner |
| **Side Effects** | Direct manipulation within loops | Direct manipulation within loops | Controlled via pure functions, explicit effects | Controlled through pure functions | Both isolate side effects in FP style |
| **Code Philosophy** | "How to do it" - step-by-step instructions | "How to do it" - step-by-step instructions | "What to do" - declarative specifications | "What to do" - declarative specifications | Rust and C++23 both support both paradigms |
| **Variable Mutability** | Heavy use of mutable variables | `let mut` required for mutable variables | Default immutable, `mut` keyword explicit | `const` variables possible but optional | Rust enforces immutability by default |
| **Function Composition** | Sequential statements, manual orchestration | Sequential statements, manual orchestration | Natural composition with `.` operator | Composable using pipe operator `\|` | Both support composition, Rust's is more intuitive |
| **Modern Language Features** | Traditional C++ style, even in C++23 | Modern Rust idioms | Idiomatic Rust with iterators | C++20 ranges, C++23 features | Both leverage modern standards |
| **Error Handling** | Error codes, exceptions built into logic | `Result<T, E>`, `Option<T>` types | `Result<T, E>`, `Option<T>` with `?` operator | `std::expected`, `std::optional` (C++23) | Rust's error handling is more integrated |
| **Parallelization** | Difficult due to shared mutable state | Difficult due to shared mutable state | Naturally parallelizable (no shared state) | Naturally parallelizable (no shared state) | Both FP styles enable safe parallelization |

---

## 🚀 Advanced Example: File Processing

### Task: Count lines in multiple files

| **Paradigm** | **C++23 Code** | **Rust Code** |
|:---|:---|:---|
| **Imperative** | `std::vector<std::string> files = {"a.txt", "b.txt"};`<br>`int total_lines = 0;`<br>`for (const auto& file : files) {`<br>&emsp;`std::ifstream f(file);`<br>`    std::string line;`<br>&emsp;`while (std::getline(f, line))`<br>&emsp;`total_lines++;`<br>`}` | `let files = vec!["a.txt", "b.txt"];`<br>`let mut total_lines = 0;`<br>`for file in files {`<br>&emsp;`let content = fs::read_to_string(file)?;`<br>&emsp;`total_lines += content.lines().count();`<br>`}`|
| **Functional** | `auto files = std::views::iota(1, 3)`<br>&emsp;`\| std::views::transform([](int i) {`<br>&emsp;`return "file" + std::to_string(i);`<br />`});` | `let total: usize = (1..3)`<br>&emsp;`.map(\|i\| format!("file{}", i))`<br>&emsp;`.map(\|name\| fs::read_to_string(name).unwrap())`<br>&emsp;`.map(\|content\| content.lines().count())`<br>&emsp;`.sum();` |

---

## 📊 Language Feature Matrix

| **Feature** | **C++23** | **Rust** | **Winner** |
|:---|:---|:---|:---|
| **Pattern Matching** | Limited (C++23 improves) | ✅ Full `match` expressions | Rust |
| **Null Safety** | ❌ Null pointers possible | ✅ `Option<T>` eliminates null | Rust |
| **Memory Safety** | ❌ Manual memory management | ✅ Ownership system | Rust |
| **Zero-Cost Abstractions** | ✅ Templates, constexpr | ✅ Monomorphization, iterators | Tie |
| **Immutability by Default** | ❌ Mutable by default | ✅ Immutable by default | Rust |
| **Error Handling** | Exceptions, error codes | `Result<T, E>` type | Context-dependent |
| **Functional Programming** | ✅ Ranges, algorithms | ✅ Iterators, closures | Tie |
| **Compile-Time Checks** | Strong type system | ✅ Borrow checker + strong types | Rust |
| **Learning Curve** | Steep (complex features) | Steep (ownership model) | Similar |

---

## 🎯 Key Takeaways

### ✅ **Rust Advantages**
1. **Safety**: Memory and thread safety guaranteed at compile time
2. **Immutability**: Default immutable variables reduce bugs
3. **Pattern Matching**: More powerful than C++23's pattern matching
4. **Error Handling**: `Result<T, E>` is more explicit than exceptions
5. **Ergonomics**: Cleaner syntax for functional operations

### ✅ **C++23 Advantages**
1. **Legacy Integration**: Better for existing C++ codebases
2. **Compile-Time**: More mature constexpr ecosystem
3. **Template Metaprogramming**: More advanced than Rust macros
4. **Industry Adoption**: Larger existing codebase and tooling

### 🔄 **When to Choose Which**
- **Choose Rust**: For new projects prioritizing safety, concurrency, and modern FP
- **Choose C++23**: For projects requiring legacy integration, advanced metaprogramming
- **Both**: Excellent for functional programming in modern systems programming

---

## 📚 Further Reading

- [Functional Programming in C++](https://www.manning.com/books/functional-programming-in-c-plus-plus) by Ivan Čukić
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Free online book
- [C++23 Ranges](https://en.cppreference.com/w/cpp/ranges)
- [Rust Iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
