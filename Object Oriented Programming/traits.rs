// Define a trait
trait Animal {
    fn speak(&self);
}

// Create a struct that implements the Animal trait
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says Woof!", self.name);
    }
}

// Create another struct that implements the Animal trait
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says Meow!", self.name);
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Rex"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };

    // Call the speak method on both Dog and Cat objects
    dog.speak();
    cat.speak();
}
