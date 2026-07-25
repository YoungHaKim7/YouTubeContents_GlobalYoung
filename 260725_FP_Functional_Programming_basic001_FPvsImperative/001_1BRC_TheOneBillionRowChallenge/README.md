# 1️⃣🐝🏎️ The One Billion Row Challenge -- A fun exploration of how quickly 1B rows from a text file can be aggregated with Java 

- https://github.com/gunnarmorling/1brc

# time 해석방법

The `time` output shows three different timing metrics for your `just cro` command:

```bash
________________________________________________________
Executed in    1.99 secs    fish           external
   usr time    3.35 secs    0.15 millis    3.35 secs
   sys time    0.55 secs    1.02 millis    0.55 secs
```

- release된 파일로 바로 실행하

- ./target/release/a.out

```bash
$  time ./target/release/a01_rust_multi_thread_1brc
________________________________________________________
Executed in   55.58 millis    fish           external
   usr time   61.82 millis    0.00 millis   61.82 millis
   sys time   31.05 millis    1.10 millis   29.95 millis
```


## Breakdown

**Wall-clock time (what you experienced)**
- `Executed in 1.99 secs` — Total elapsed time from start to finish

**CPU time breakdown**
- `usr time 3.35 secs` — Time CPU spent executing **your program's code** (user-space)
- `sys time 0.55 secs` — Time CPU spent in **kernel** doing system calls, I/O, etc.

## What the numbers tell you

```
usr (2.69s) + sys (0.28s) ≈ 2.97s
real (2.96s) ≈ 2.96s
```

Since **usr + sys ≈ real**, your program was **CPU-bound** — it was actively using the CPU most of the time, not waiting on disk/network.

### Other scenarios you might see:

| Pattern            | Meaning                                                                   |
| ------------------ | ------------------------------------------------------------------------- |
| `usr + sys > real` | **Parallel processing** — multiple cores working simultaneously           |
| `usr + sys ≪ real` | **I/O-bound** — program spent time waiting for disk, network, or sleeping |
| `sys ≈ usr`        | Heavy system call usage (lots of file operations, network, etc.)          |
| `sys ≈ 0`          | Pure computation, minimal OS interaction                                  |

## Quick rule of thumb
- **Real time** = How long you waited
- **User time** = Your code running
- **System time** = The kernel helping your code
