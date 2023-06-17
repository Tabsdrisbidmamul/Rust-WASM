fn main() {
    let mut person = Person::new();
    let person_2 = Person::from("Test", "test", 3);

    Person::some_fn();
    person.change_age(100);
    person.change_first_name("Idris");
    person.change_last_name("Khan");

    println!("{} {} {}", person.first_name, person.last_name, person.age);

    println!(
        "{} {} {}",
        person_2.first_name, person_2.last_name, person_2.age
    );
}

struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn new() -> Person {
        return Person {
            first_name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
        };
    }

    fn from(first_name: &str, last_name: &str, age: u32) -> Person {
        return Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
        };
    }

    fn some_fn() {
        println!("some_fn()")
    }

    fn change_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    fn change_first_name(&mut self, new_first_name: &str) {
        self.first_name = new_first_name.to_string();
    }

    fn change_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string();
    }
}
