use std::thread;

fn main() {
    let my_cpu_num = thread::available_parallelism().unwrap();
    println!("You can use {:?} threads available no.", my_cpu_num);
}
