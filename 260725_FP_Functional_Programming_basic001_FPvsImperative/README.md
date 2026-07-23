# link

<hr />

# Marp ppt
- https://www.npmjs.com/package/@marp-team/marp-cli


```bash
npm install --save-dev @marp-team/marp-cli
```

- Watch mode

```bash
npx @marp-team/marp-cli@latest -w FP_vs_Imperative.md
```

- Convert slide deck into HTML

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
