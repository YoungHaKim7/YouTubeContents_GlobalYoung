use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    let person2 = Person {
        name: "Gyoung".to_string(),
        age: 40,
    };

    let db = Arc::new([person, person2]);
    // let db = Arc::new(Mutex::new([person, person2]));
    // let db = vec![person, person2];
    println!("Arc db : {db:?}");

    // let db2 = db.lock().unwrap();

    // for item in db2.iter() {
    //     println!("{item:?}");
    // }
    // println!("db2 {db2:?}");

    // for dbs in db.iter() {
    //     println!("db {dbs:?}");
    // }
}
