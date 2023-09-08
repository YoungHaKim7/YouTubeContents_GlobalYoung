use std::{sync::Arc, thread};

struct Person {
    name: Arc<String>,
    age: Arc<u32>,
}

fn main() {
    let person = Person {
        name: Arc::new("John Doe".to_string()),
        age: Arc::new(30),
    };

    // let arc_person = Arc::new(person);

    // Share the reference to the person between multiple threads.
    let mut t1 = thread::spawn(move || {
        println!("The person's name is {}", person.name);
    });

    let mut t2 = thread::spawn(move || {
        println!("The person's age is {}", person.age);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
