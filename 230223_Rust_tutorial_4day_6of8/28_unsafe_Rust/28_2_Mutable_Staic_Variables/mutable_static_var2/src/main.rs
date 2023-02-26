static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    } // Potential data race!!!
}

fn main() {
    add_to_counter(42);
    //    let y = add_to_counter(42);
    //    let z = add_to_counter(42);

    unsafe {
        println!("COUNTER: {COUNTER}");
        //        println!("y = {y:?}");
        //        println!("z = {z:?}");
    }
}
