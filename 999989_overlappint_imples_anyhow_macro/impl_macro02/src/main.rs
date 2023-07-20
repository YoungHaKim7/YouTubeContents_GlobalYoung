use std::fs;

use anyhow::*;

fn demo1() -> Result<(), anyhow::Error> {
    // Turn a &str into an error.
    // &str implements Display but not std::error::Error.
    return Err(anyhow!("oh no!"));
}

fn demo2() -> Result<(), anyhow::Error> {
    // Turn an existing std::error::Error value into our error without
    // losing its source() and backtrace() if there is one.
    let io_error = fs::read("/tmp/nonexist").unwrap_err();
    return Err(anyhow!(io_error));
}

fn main() {
    let _ = demo1();
    let _ = demo2();
}
