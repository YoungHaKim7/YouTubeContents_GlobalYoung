// https://stackoverflow.com/questions/76535332/is-it-possible-to-convert-a-t-or-vect-to-arcmutext
use std::sync::{Arc, Mutex};

fn arc_mutex_from_slice_clone<T: Clone>(v: &[T]) -> Arc<Mutex<Box<[T]>>> {
    let owned = v.to_owned();
    let boxed = owned.into_boxed_slice();
    Arc::new(Mutex::new(boxed))
}

fn arc_mutex_from_vec<T>(v: Vec<T>) -> Arc<Mutex<Box<[T]>>> {
    let boxed = v.into_boxed_slice();
    Arc::new(Mutex::new(boxed))
}

fn arc_mutex_sized_from_vec<T: std::fmt::Debug, const N: usize>(v: Vec<T>) -> Arc<Mutex<[T; N]>> {
    let array = v.try_into().unwrap();
    Arc::new(Mutex::new(array))
}

fn main() {
    let v1 = vec![1, 2, 3];
    let m1 = arc_mutex_from_slice_clone(v1.as_slice());
    println!("m1: {:?}", m1);
    // v1 is still usable since m1 contains a clone
    println!("v: {:?}", v1);
    let m2 = arc_mutex_from_vec(v1);
    println!("m2: {:?}", m2);
    // v1 is not usable anymore since is it moved int m2
    // println!("v: {:?}", v1);
    //
    let v2 = vec![6, 7, 8, 9];
    let m3 = arc_mutex_sized_from_vec::<_, 4>(v2);
    println!("m3: {:?}", m3);
}
/*
m1: Mutex { data: [1, 2, 3], poisoned: false, .. }
v: [1, 2, 3]
m2: Mutex { data: [1, 2, 3], poisoned: false, .. }
m3: Mutex { data: [6, 7, 8, 9], poisoned: false, .. }
*/
