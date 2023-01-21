# Result:

```
$ cargo run

   Compiling luhn_algorithm v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230121_Rust_tutorial_4day_4of8/luhn_algorithm)
    Finished dev [unoptimized + debuginfo] target(s) in 3.40s
     Running `target/debug/luhn_algorithm`

Is 1234 5678 1234 5670 a valid credit card number? yes


$ cargo run
   Compiling luhn_algorithm v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230121_Rust_tutorial_4day_4of8/luhn_algorithm)
    Finished dev [unoptimized + debuginfo] target(s) in 3.40s
     Running `target/debug/luhn_algorithm`

Is 1234 5678 1234 5670 a valid credit card number? yes


$ cargo test
   Compiling luhn_algorithm v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230121_Rust_tutorial_4day_4of8/luhn_algorithm)
    Finished test [unoptimized + debuginfo] target(s) in 0.16s
     Running unittests src/main.rs (target/debug/deps/luhn_algorithm-68a15f25245cf0d3)

running 6 tests
test test_empty_cc_number ... ok
test test_invalid_digit_cc_number ... ok
test test_single_digit_cc_number ... ok
test test_non_digit_cc_number ... ok
test test_valid_digit_cc_number ... ok
test test_two_digit_cc_number ... FAILED

failures:

---- test_two_digit_cc_number stdout ----
thread 'test_two_digit_cc_number' panicked at 'assertion failed: !luhn(\" 0 0 \")', src/main.rs:56:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_two_digit_cc_number

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin luhn_algorithm`
cargo test --bin luhn_algorithm
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/main.rs (target/debug/deps/luhn_algorithm-68a15f25245cf0d3)

running 6 tests
test test_invalid_digit_cc_number ... ok
test test_non_digit_cc_number ... ok
test test_single_digit_cc_number ... ok
test test_empty_cc_number ... ok
test test_two_digit_cc_number ... FAILED
test test_valid_digit_cc_number ... ok

failures:

---- test_two_digit_cc_number stdout ----
thread 'test_two_digit_cc_number' panicked at 'assertion failed: !luhn(\" 0 0 \")', src/main.rs:56:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_two_digit_cc_number

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin luhn_algorithm`
cargo run --bin luhn_algorithm
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/luhn_algorithm`
Is 1234 5678 1234 5670 a valid credit card number? yes


$ cargo nextest run

    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 6 tests across 1 binaries
        PASS [   0.004s] luhn_algorithm::bin/luhn_algorithm test_empty_cc_number
        PASS [   0.004s] luhn_algorithm::bin/luhn_algorithm test_invalid_digit_cc_number
        PASS [   0.006s] luhn_algorithm::bin/luhn_algorithm test_non_digit_cc_number
        PASS [   0.006s] luhn_algorithm::bin/luhn_algorithm test_single_digit_cc_number
        PASS [   0.007s] luhn_algorithm::bin/luhn_algorithm test_valid_digit_cc_number
        FAIL [   0.007s] luhn_algorithm::bin/luhn_algorithm test_two_digit_cc_number

--- STDOUT:              luhn_algorithm::bin/luhn_algorithm test_two_digit_cc_number ---

running 1 test
test test_two_digit_cc_number ... FAILED

failures:

failures:
    test_two_digit_cc_number

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s


--- STDERR:              luhn_algorithm::bin/luhn_algorithm test_two_digit_cc_number ---
thread 'test_two_digit_cc_number' panicked at 'assertion failed: !luhn(\" 0 0 \")', src/main.rs:56:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

   Canceling due to test failure: 0 tests still running
------------
     Summary [   0.008s] 6 tests run: 5 passed, 1 failed, 0 skipped
error: test run failed


$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/luhn_algorithm`

Is 1234 5678 1234 5670 a valid credit card number? yes


$ cargo test
   Compiling luhn_algorithm v0.1.0 (/Users/globalyoung/Documents/test/test/rust/YouTubeContents_GlobalYoung/230121_Rust_tutorial_4day_4of8/luhn_algorithm)
    Finished test [unoptimized + debuginfo] target(s) in 0.16s
     Running unittests src/main.rs (target/debug/deps/luhn_algorithm-68a15f25245cf0d3)

running 6 tests
test test_empty_cc_number ... ok
test test_invalid_digit_cc_number ... ok
test test_single_digit_cc_number ... ok
test test_non_digit_cc_number ... ok
test test_valid_digit_cc_number ... ok
test test_two_digit_cc_number ... FAILED

failures:

---- test_two_digit_cc_number stdout ----
thread 'test_two_digit_cc_number' panicked at 'assertion failed: !luhn(\" 0 0 \")', src/main.rs:56:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_two_digit_cc_number

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin luhn_algorithm`
cargo test --bin luhn_algorithm
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/main.rs (target/debug/deps/luhn_algorithm-68a15f25245cf0d3)

running 6 tests
test test_invalid_digit_cc_number ... ok
test test_non_digit_cc_number ... ok
test test_single_digit_cc_number ... ok
test test_empty_cc_number ... ok
test test_two_digit_cc_number ... FAILED
test test_valid_digit_cc_number ... ok

failures:

---- test_two_digit_cc_number stdout ----
thread 'test_two_digit_cc_number' panicked at 'assertion failed: !luhn(\" 0 0 \")', src/main.rs:56:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_two_digit_cc_number

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin luhn_algorithm`
cargo run --bin luhn_algorithm
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/luhn_algorithm`
Is 1234 5678 1234 5670 a valid credit card number? yes
cargo nextest run
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 6 tests across 1 binaries
        PASS [   0.004s] luhn_algorithm::bin/luhn_algorithm test_empty_cc_number
        PASS [   0.004s] luhn_algorithm::bin/luhn_algorithm test_invalid_digit_cc_number
        PASS [   0.006s] luhn_algorithm::bin/luhn_algorithm test_non_digit_cc_number
        PASS [   0.006s] luhn_algorithm::bin/luhn_algorithm test_single_digit_cc_number
        PASS [   0.007s] luhn_algorithm::bin/luhn_algorithm test_valid_digit_cc_number
        FAIL [   0.007s] luhn_algorithm::bin/luhn_algorithm test_two_digit_cc_number

--- STDOUT:              luhn_algorithm::bin/luhn_algorithm test_two_digit_cc_number ---

running 1 test
test test_two_digit_cc_number ... FAILED

failures:

failures:
    test_two_digit_cc_number

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s


--- STDERR:              luhn_algorithm::bin/luhn_algorithm test_two_digit_cc_number ---
thread 'test_two_digit_cc_number' panicked at 'assertion failed: !luhn(\" 0 0 \")', src/main.rs:56:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

   Canceling due to test failure: 0 tests still running
------------
     Summary [   0.008s] 6 tests run: 5 passed, 1 failed, 0 skipped
error: test run failed


$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/luhn_algorithm`

Is 1234 5678 1234 5670 a valid credit card number? yes
```
