fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and register it for benchmarking.
#[divan::bench]
fn fibonacci() -> u64 {
    fn compute(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            compute(n - 2) + compute(n - 1)
        }
    }

    compute(divan::black_box(15))
}

#[divan::bench]
fn fibonacci_recursive_compute() -> u64 {
    fn compute(n: u64) -> u64 {
        let count = 15;
        let mut a = 0u64;
        let mut b = 1u64;
        for c in 1..=count {
            let next = a + b;
            if c % 2 == 0 {
                a = next;
            } else {
                b = next;
            }
        }
        if count % 2 == 0 {
            a
        } else {
            b
        }
    }
    compute(divan::black_box(15))
}

#[divan::bench]
fn fibonacci_iterator_compute() -> u64 {
    fn compute(n: u64) -> u64 {
        let count = 15;
        let mut a = 0u64;
        let mut b = 1u64;
        { 1..=count }.for_each(|c| {
            let next = a + b;
            if c % 2 == 0 {
                a = next;
            } else {
                b = next;
            }
        });
        if count % 2 == 0 {
            a
        } else {
            b
        }
    }
    compute(divan::black_box(15))
}

#[divan::bench]
fn fibonacci_recursive() -> u64 {
    let count = 15;
    let mut a = 0u64;
    let mut b = 1u64;
    for c in 1..=count {
        let next = a + b;
        if c % 2 == 0 {
            a = next;
        } else {
            b = next;
        }
    }
    if count % 2 == 0 {
        a
    } else {
        b
    }
}

#[divan::bench]
fn fibonacci_iterator() -> u64 {
    let count = 15;
    let mut a = 0u64;
    let mut b = 1u64;
    { 1..=count }.for_each(|c| {
        let next = a + b;
        if c % 2 == 0 {
            a = next;
        } else {
            b = next;
        }
    });
    if count % 2 == 0 {
        a
    } else {
        b
    }
}
