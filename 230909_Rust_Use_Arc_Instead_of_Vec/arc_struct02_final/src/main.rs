use std::{
    mem,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct TwoArrays {
    pub a: Arc<Vec<i32>>,
    pub b: Arc<Vec<i32>>,
}

fn add_arrays(mut foo: TwoArrays) {
    let a = foo.a.clone();

    let mut b_new = vec![];
    for i in a.iter() {
        b_new.push(i + foo.b[0]);
    }

    // Swap the contents of b with b_new.
    // mem::swap(&mut foo.b, &mut b_new);

    println!("Arrays A: {:?}", &foo.a);
    println!("Arrays A: {:?}", &foo.b);
}

fn main() {
    let a = vec![1i32, 2i32, 3i32];
    let b = vec![4i32, 5i32, 6i32];
    let two_arrays = TwoArrays {
        a: a.into(),
        b: b.into(),
    };
    let foo = Arc::new(Mutex::new(&two_arrays));

    add_arrays(two_arrays);
}
