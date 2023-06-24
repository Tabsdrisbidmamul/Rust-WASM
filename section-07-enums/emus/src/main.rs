fn main() {
    let person = Person::from("Idris", "Khan", 23, PersonIdentity::Passport);

    println!(
        "{} {} {} {:?}",
        person.first_name, person.last_name, person.age, person.id
    )
}

#[derive(Debug)]
enum PersonIdentity {
    Passport,
    IdentityCard,
}

struct Person {
    first_name: String,
    last_name: String,
    age: u32,
    id: PersonIdentity,
}

impl Person {
    fn new() -> Person {
        return Person {
            first_name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonIdentity::Passport,
        };
    }

    fn from(first_name: &str, last_name: &str, age: u32, id: PersonIdentity) -> Person {
        return Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            id,
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
