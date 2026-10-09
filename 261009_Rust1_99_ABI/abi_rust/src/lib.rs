// src/lib.rs

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_sub(a: i32, b: i32) -> i32 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_mul(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rust_ffi_add(2, 2);
        assert_eq!(result, 4);
        let result = rust_ffi_sub(6, 2);
        assert_eq!(result, 4);
        let result = rust_ffi_mul(2, 2);
        assert_eq!(result, 4);
    }
}
