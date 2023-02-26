use std::mem::size_of_val;
use std::slice;

pub unsafe trait AsBytes {
    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, size_of_val(self)) }
    }
}

unsafe impl AsBytes for u32 {}

fn main() {
    let x = 42;
    let ptr = &x as *const _;
    let slice = unsafe { slice::from_raw_parts(ptr, 1) };

    println!("slice[0]: {}", slice[0]);
}
