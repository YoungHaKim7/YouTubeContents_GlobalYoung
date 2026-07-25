# Result

- flamegraph분석

```bash
$ CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --release --bin a01_rust_multi_thread_1brc

[ perf record: Woken up 11 times to write data ]
[ perf record: Captured and wrote 5.725 MB perf.data (96 samples) ]
Running perf script [11s]:                                                                                                                                         writing flamegraph to "flamegraph.svg"
```

- hyperfine 분석

```bash
$ hyperfine \
          'cargo r --release' \
          'RAYON_NUM_THREADS=1 cargo r --release' \
          'RAYON_NUM_THREADS=4 cargo r --release' \
          --warmup 3
Benchmark 1: cargo r --release
  Time (mean ± σ):      42.6 ms ±   2.5 ms    [User: 42.5 ms, System: 15.7 ms]
  Range (min … max):    39.1 ms …  51.7 ms    68 runs

Benchmark 2: RAYON_NUM_THREADS=1 cargo r --release
  Time (mean ± σ):      47.1 ms ±   2.3 ms    [User: 37.9 ms, System: 9.6 ms]
  Range (min … max):    43.2 ms …  55.2 ms    62 runs

Benchmark 3: RAYON_NUM_THREADS=4 cargo r --release
  Time (mean ± σ):      43.3 ms ±   2.3 ms    [User: 39.8 ms, System: 10.8 ms]
  Range (min … max):    40.1 ms …  48.4 ms    61 runs

Summary
  cargo r --release ran
    1.02 ± 0.08 times faster than RAYON_NUM_THREADS=4 cargo r --release
    1.11 ± 0.08 times faster than RAYON_NUM_THREADS=1 cargo r --release
```


```bash
$ time cargo run --release

________________________________________________________
Executed in   80.06 millis    fish           external
   usr time   61.93 millis    0.04 millis   61.89 millis
   sys time   33.95 millis    1.01 millis   32.94 millis


```

# entry mothod

- https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.entry

```rs
use std::collections::HashMap;

let mut letters = HashMap::new();

for ch in "a short treatise on fungi".chars() {
    letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

<hr />

Perfect! The code is now working correctly. Here's what I fixed:

## Summary of Changes

1. **Renamed struct and fields**: Changed `CountryStats` → `StationStats` with temperature-specific fields:
   - `min_data/max_data/sum_data` → `min_temp/max_temp/sum_temp`
   - Changed data types from `u64` to `f64` to handle decimal temperatures

2. **Updated parsing logic**: Changed from CSV format to 1BRC format (`station;temperature`):
   - Split by semicolon `;` instead of comma
   - Parse temperatures as `f64` instead of populations as `u64`
   - Handle negative temperatures (e.g., `-12.5`)

3. **Renamed variables throughout**: Updated all variable names from `country`/`population` to `station`/`temperature` for clarity

4. **Updated output formatting**: Changed to show 1 decimal place for temperatures (`{:.1}`)

The program successfully:
- ✅ Reads 44,693 lines from the file
- ✅ Processes them in parallel using 20 chunks with 20 threads  
- ✅ Calculates min/mean/max temperatures for 41,343 unique weather stations
- ✅ Outputs results in the standard 1BRC format: `{station=min/mean/max, ...}`

The code is now correctly adapted for the One Billion Row Challenge temperature measurement task!


# `time` 성능 측정

- Perfect! Now let me give you a **comprehensive analysis** of your timing data with full context:

---

## 📊 Complete Timing Analysis

### Your Original Output:
```
Executed in   32.39 millis    fish           external
   usr time   25.02 millis    0.00 micros   25.02 millis
   sys time   23.58 millis  740.00 micros   22.84 millis
```

### Breaking Down Each Field:

#### **Row 1: `Executed in 32.39 millis`**
- **What it is**: Wall-clock time (elapsed real time)
- **Meaning**: The actual time that passed from start to finish
- **Perception**: This is what YOU experience waiting for the program

#### **Row 2: `usr time 25.02 millis`**
- **What it is**: CPU time spent in **user-mode** (your code)
- **Breakdown**:
  - `25.02 millis` - Total user CPU time
  - `0.00 micros` - Time spent in signal handlers (negligible here)
  - `25.02 millis` - User time repeated

#### **Row 3: `sys time 23.58 millis`**
- **What it is**: CPU time spent in **kernel-mode** (system calls)
- **Breakdown**:
  - `23.58 millis` - Total system CPU time
  - `740.00 micros` - Time in signal handlers (0.74ms)
  - `22.84 millis` - System time excluding signals (23.58 - 0.74)

---

## 🔍 Key Performance Insights

### **1. Multi-threading Detection**
```
User + System = 25.02 + 23.58 = 48.60 ms
Elapsed time = 32.39 ms

CPU Utilization = 48.60 / 32.39 = 1.50x (150%)
```

**This proves parallel processing!** If single-threaded, User + System ≤ Elapsed. Since 48.60 > 32.39, your program used multiple cores simultaneously.

### **2. What Your Program Does**
- **Input**: `assets/weather_stations.csv` (806KB, 44,693 lines)
- **Format**: `CityName;temperature` (e.g., `Tokyo;35.6897`)
- **Processing**: Calculates min/max/mean temperatures per station
- **Parallelization**: Uses Rayon to process chunks in parallel

### **3. System Context**
- **Your CPU**: 20 cores (40 with hyperthreading)
- **Rayon default threads**: Usually equals logical cores (20-40)
- **Effective parallelism**: ~1.5x speedup (not 20x because file I/O and overhead dominate)

---

## 📈 Why Isn't It Faster?

### **Bottlenecks in Your Current Implementation:**

1. **File I/O is sequential** - Reading the file happens before parallelization
2. **Small dataset** - 44K lines × ~30 bytes = ~1.3MB (fits in CPU cache)
3. **Thread overhead** - Spawning threads has overhead that outweighs benefits for small data
4. **Memory allocation** - `lines.collect()` copies everything into memory first

### **Code Evidence:**
```rust
// Lines 98-99: This is SEQUENTIAL and slow for large files
let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
```

For 1BRC (1 billion rows), this approach would **consume 30-60GB of RAM**!

---

## 🧪 Better Performance Analysis Commands

### **Quick comparison script:**
```bash
# Create this benchmark script
cat > benchmark.sh << 'EOF'
#!/bin/bash
echo "=== Thread Scaling Analysis ==="
echo "Threads | Real(ms) | User(ms) | Sys(ms) | Speedup"
echo "--------|----------|----------|---------|--------"
for threads in 1 2 4 8 16 32; do
    output=$({ time RAYON_NUM_THREADS=$threads cargo r --release > /dev/null; } 2>&1)
    real=$(echo "$output" | grep "real" | awk '{print $2}' | sed 's/0m//;s/s//')
    user=$(echo "$output" | grep "user" | awk '{print $2}' | sed 's/0m//;s/s//')
    sys=$(echo "$output" | grep "sys" | awk '{print $2}' | sed 's/0m//;s/s//')
    echo "$threads | $real | $user | $sys | ..."
done
EOF
chmod +x benchmark.sh
./benchmark.sh
```

### **Use proper benchmarking tools:**
```bash
# Install hyperfine (recommended)
cargo install hyperfine

# Compare different implementations
hyperfine \
    'cargo r --release' \
    'RAYON_NUM_THREADS=1 cargo r --release' \
    'RAYON_NUM_THREADS=4 cargo r --release' \
    --warmup 3
```

### **Profile where time is spent:**
```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --release --bin rust_multi_thread_1brc

# This creates flamegraph.svg showing exactly where CPU time goes
```

---

## 📝 Summary Metrics

| Metric             | Value             | Meaning             |
| ------------------ | ----------------- | ------------------- |
| **Total work**     | 48.60 ms          | All CPU time needed |
| **Parallel time**  | 32.39 ms          | Wall-clock elapsed  |
| **Speedup**        | 1.50x             | Due to threading    |
| **Efficiency**     | 1.50/20 = 7.5%    | Low! (I/O bound)    |
| **Data processed** | 44,693 records    | Per run             |
| **Throughput**     | ~1,380 records/ms | ~1.4M records/sec   |

---

## 🎯 Key Takeaways

1. **Your timing is correct** - The multi-threading is working (usr+sys > elapsed)
2. **But efficiency is low** - Only 1.5x speedup on 20-core machine
3. **Bottleneck is I/O** - File reading dominates, not computation
4. **For true 1BRC** - Need streaming/chunked reading, not `.collect()`
5. **Your program is fast** - 32ms is excellent for this demo dataset!

Would you like me to show you how to optimize for true billion-row performance, or analyze any specific aspect in more detail?
