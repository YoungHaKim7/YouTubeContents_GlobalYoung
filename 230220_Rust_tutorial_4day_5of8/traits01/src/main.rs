trait Greet {
    fn say_hello(&self);
}

struct Dog {
    name: String,
}

struct Cat;

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Wof, my name is {}!", self.name);
    }
}

impl Greet for Cat {
    fn say_hello(&self) {
        println!("Miau!");
    }
}

fn main() {
    let pets: Vec<Box<dyn Greet>> = vec![
        Box::new(Dog {
            name: String::from("Fido"),
        }),
        Box::new(Cat),
    ];
    for pet in pets {
        pet.say_hello();
    }

    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{} ", std::mem::size_of::<&dyn Greet>());
    println!("{} ", std::mem::size_of::<Box<&dyn Greet>>());
}
