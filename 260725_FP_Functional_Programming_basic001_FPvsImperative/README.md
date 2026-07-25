# link
- 코드 이쁘게 highlightjs 테마
  - https://github.com/highlightjs/highlight.js/tree/main/src/styles

<hr />

# Marp ppt
- https://www.npmjs.com/package/@marp-team/marp-cli

## install(local)

```bash
npm install --save-dev @marp-team/marp-cli
```

## 내가 주로 쓰는법(`npx` 최고)

- Watch mode(`npx`의 장점은 설치 없이 바로 가능한점, 내가 수정하면서 보고 싶으면 바로 이거)

```bash
npx @marp-team/marp-cli@latest -w FP_vs_Imperative.md
```

- Convert slide deck into HTML(`npx`의 장점은 설치 없이 바로 가능한점)

```bash
npx @marp-team/marp-cli@latest FP_vs_Imperative.md
```


# Comprehensive Analysis: Imperative vs Functional Programming[|🔝|](#link)

## 📊 Detailed Comparison Table(C++23으로 비교)

| **Aspect** | **Imperative** | **Functional** | **Key Differences** |
|------------|----------------------|--------------------------|-------------------|
| **Control Flow** | Explicit for/while loops with manual iteration | Algorithm-based (`std::transform`, `std::for_each`) | FP abstracts iteration control |
| **State Management** | Manual state accumulation in mutable variables | Declarative transformations with minimal state | FP reduces mutability |
| **Iteration Style** | `for (auto& file : files)` loop control | `files \| views::transform(func)` abstraction | FP uses iterator composition |
| **Side Effects** | Direct manipulation of state within loops | Controlled side effects through pure functions | FP isolates side effects |
| **Code Philosophy** | "How to do it" - step-by-step instructions | "What to do" - declarative specifications | Different mental models |
| **Variable Mutability** | Heavy use of `vector<int> results`, `int line_count` | Immutable where possible, const variables | FP emphasizes immutability |
| **Function Composition** | Sequential statements, manual orchestration | Composable operations using pipes/chains | FP enables compositionality |
| **Modern C++ Usage** | Traditional C++ style (pre-C++20) | C++20 ranges, C++23 features | FP leverages modern standards |
| **Error Handling** | Manual checking, error codes built into logic | Can be integrated with `std::expected`, `std::optional` | FP has better error handling |
| **Parallelization** | Difficult to parallelize due to shared state | Naturally parallelizable (no shared state) | FP better suited for concurrency |


Perfect! I've created comprehensive comparison tables with Rust code in the middle, comparing all three approaches. Here's what I created:

## 📊 Files Created

### 1. **RUST_CPP_COMPARISON.md** - Comprehensive detailed comparison
- Code examples for both imperative and functional styles
- Advanced examples (file processing, iterator chains)
- Language feature matrix comparing all aspects
- Detailed analysis of advantages/disadvantages

### 2. **COMPARISON_TABLE.md** - Matches your original format
- Your exact table structure with Rust added as middle columns
- Side-by-side code examples
- Clear comparisons of each paradigm

## 🎯 Key Comparisons Highlighted

The tables show Rust positioned between C++23 Imperative and Functional:

| **Aspect**       | **C++23 Imperative**               | **Rust Imperative**              | **Rust Functional**                    | **C++23 Functional**             |
| ---------------- | ---------------------------------- | -------------------------------- | -------------------------------------- | -------------------------------- |
| **Code Example** | `for (int i = 1; i <= 10; ++i)`    | `for i in 1..=10`                | `(1..=10).fold(0, \|acc,x\| acc+x)`    | `std::ranges::fold_left(...)`    |
| **Mutability**   | `int sum = 0` (mutable by default) | `let mut sum = 0` (explicit mut) | `let sum = ...` (immutable by default) | `int sum = ...` (const possible) |
| **Range Syntax** | Manual loops                       | Range `1..=10`                   | Range `1..=10`                         | `std::views::iota(1, 11)`        |

## 🔍 Key Insights

**Rust Advantages:**
- More concise functional syntax (`fold` vs `fold_left`)
- Immutability by default (safety-first design)
- Pattern matching with `match` expressions
- `Result<T, E>` for explicit error handling
- Zero-cost abstractions with iterators

**C++23 Advantages:**
- Legacy codebase integration
- Advanced template metaprogramming
- Larger industry ecosystem
- More flexible paradigm mixing

Both languages excel at functional programming - Rust with modern safety features, C++23 with powerful ranges and algorithms.
