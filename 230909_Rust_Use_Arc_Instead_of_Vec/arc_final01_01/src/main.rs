use std::sync::Arc;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
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
    // let arc_person = Arc::new(person);

    for dbs in db.iter() {
        println!("db {dbs:?}");
    }

    // for dbs in db.iter().by_ref() {
    //     if let Some(dbs) = db.iter().next() {
    //         println!("db {dbs:?}");
    //     }
    // }

    // // Share the reference to the person between multiple threads.
    // let mut t1 = thread::spawn(move || {
    //     println!("The person's name is {}", db);
    // });

    // let mut t2 = thread::spawn(move || {
    //     println!("The person's age is {}", db.person2);
    // });

    // t1.join().unwrap();
    // t2.join().unwrap();
}
