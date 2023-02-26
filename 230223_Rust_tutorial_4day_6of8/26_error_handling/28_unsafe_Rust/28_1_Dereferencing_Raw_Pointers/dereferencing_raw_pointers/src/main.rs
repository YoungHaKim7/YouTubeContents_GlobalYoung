fn main() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is : {}", *r2)
    }
}
