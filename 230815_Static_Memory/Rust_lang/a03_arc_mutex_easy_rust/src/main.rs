// Mutex & Mutual exclusion
// interior mutability

use std::{
    sync::{Arc, Mutex},
    thread,
};

trait AddCount {
    fn add_fuction(&self);
}

#[derive(Debug)]
struct OurStruct {
    data: Arc<Mutex<u8>>,
}

impl AddCount for OurStruct {
    fn add_fuction(&self) {
        *self.data.lock().unwrap() += 1;
    }
}

fn main() {
    let our_struct = OurStruct {
        data: Arc::new(Mutex::new(0)),
    };

    let mut join_vec = vec![];
    for _ in 0..10 {
        let join_handle = thread::scope(|_| {
            *our_struct.data.lock().unwrap() += 1;
            println!("our struct data =  {:?}", our_struct);
        });
        join_vec.push(join_handle);
    }

    for handle in join_vec {
        handle
    }

    println!("my data : {our_struct:?}");
}
