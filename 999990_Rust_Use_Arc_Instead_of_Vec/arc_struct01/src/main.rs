// https://stackoverflow.com/questions/54942045/accessing-two-vectors-in-a-struct-locked-by-a-mutex
use std::mem;

#[derive(Debug)]
pub struct TwoArrays {
    pub a: Vec<i32>,
    pub b: Vec<i32>,
}

fn add_arrays(mut foo: TwoArrays) {
    let a = foo.a.clone();
    let mut b = foo.b.clone();

    for i in a.iter() {
        let mut index = 0;
        for _j in b.iter_mut() {
            let mut new_value = i.clone() + foo.b[index as usize].clone();
            mem::swap(&mut foo.b[index as usize], &mut new_value);
            index = index + 1;
        }
    }

    println!("Arrays A: {:?}", &foo.a);
    println!("Arrays A: {:?}", &foo.b);
}

fn main() {
    let a = vec![1i32, 2i32, 3i32];
    let b = vec![4i32, 5i32, 6i32];
    let two_arrays = TwoArrays { a, b };
    // let foo = Arc::new(Mutex::new(two_arrays));

    add_arrays(two_arrays);
}
