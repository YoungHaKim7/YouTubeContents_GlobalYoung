struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    // let arc_person = Arc::new(person);

    // Share the reference to the person between multiple threads.
    println!("The person's name is {}", person.name);

    println!("The person's age is {}", person.age);
}
