use pi_arr::*;

fn main() {
    let arr = arr![1, 2, 3];

    println!("arr = {:?}", arr);
    let mut my_arr_clone = arr.clone();
    my_arr_clone.set(3, 40);
    println!("arr = {:?}", my_arr_clone);
}
