static mut COUNT: u32 = 0;

fn fuction_with_static_variable(count: &mut u32) {
    *count += 1;
    println!("{}", *count);
}

fn main() {
    unsafe {
        fuction_with_static_variable(&mut COUNT);
        fuction_with_static_variable(&mut COUNT);
        fuction_with_static_variable(&mut COUNT);
        fuction_with_static_variable(&mut COUNT);
    }
}
