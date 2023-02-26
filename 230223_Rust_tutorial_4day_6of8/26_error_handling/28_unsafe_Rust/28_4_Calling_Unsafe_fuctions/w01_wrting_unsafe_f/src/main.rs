unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut a = 42;
    let mut b = 66;

    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {a}, b = {b}");
}
