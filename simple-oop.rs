struct Animal {
    name: String,
    age: u32,
}

impl Animal {
    fn new(name: String, age: u32) -> Self {
        Animal { name, age }
    }

    fn make_sound(&self) -> String {
        String::from("Some generic sound")
    }
}

struct Dog {
    animal: Animal,
    breed: String,
}

impl Dog {
    fn new(name: String, age: u32, breed: String) -> Self {
        Dog {
            animal: Animal::new(name, age),
            breed,
        }
    }

    fn make_sound(&self) -> String {
        String::from("Woof!")
    }

    fn get_info(&self) -> String {
        format!(
            "{} is a {} dog, {} years old",
            self.animal.name, self.breed, self.animal.age
        )
    }
}

fn main() {
    let dog = Dog::new(String::from("Rex"), 3, String::from("Labrador"));
    println!("{}", dog.get_info());
    println!("{}", dog.make_sound());
}