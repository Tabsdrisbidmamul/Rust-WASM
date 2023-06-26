fn main() {
    let person = Person::from(
        "Idris",
        "Khan",
        23,
        PersonIdentity::Passport("123456".to_string()),
    );

    let person_2 = Person::from(
        "Test",
        "test last name",
        0,
        PersonIdentity::IdentityCard("123456".to_string()),
    );

    let animal = Animal("Dog".to_string(), 10, "Bulldog".to_string());

    animal.display_info();

    person.display_info();

    let is_id_match = person.check_person_id(&person_2);

    println!("main(): is_id_match {}", is_id_match);

    person.alert();
    animal.alert();

    log_info_dyn(&person);
    log_info(person);
}

fn log_info(value: impl Log) {
    value.alert();
}

fn log_info_dyn(value: &dyn Log) {
    value.alert()
}

trait Log {
    fn display_info(&self);
    fn alert(&self) {
        println!("ALERT")
    }
}

#[derive(Eq, PartialEq)]
enum PersonIdentity {
    Passport(String),
    IdentityCard(String),
}

struct Person {
    first_name: String,
    last_name: String,
    age: u32,
    id: PersonIdentity,
}

struct Animal(String, u32, String);

impl Log for Animal {
    fn display_info(&self) {
        println!("{} {} {}", self.0, self.1, self.2)
    }

    fn alert(&self) {
        println!("ANIMAL: ALERT")
    }
}

impl Log for Person {
    fn display_info(&self) {
        println!(
            "{} {} {} {}",
            self.first_name,
            self.last_name,
            self.age,
            self.get_identity_value()
        )
    }
}

impl Person {
    fn from(first_name: &str, last_name: &str, age: u32, id: PersonIdentity) -> Person {
        return Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            id,
        };
    }

    fn get_identity_value(&self) -> &String {
        let value = match &self.id {
            PersonIdentity::IdentityCard(s) => s,
            PersonIdentity::Passport(s) => s,
        };

        return value;
    }

    fn check_person_id(&self, to_match: &Person) -> bool {
        let self_id = self.get_identity_value();
        let to_match_id = to_match.get_identity_value();

        let is_id_type_match = self.id == to_match.id;

        let is_id_match = self_id == to_match_id;

        println!("check_person_id(): is_id_type_match {}", is_id_type_match);

        println!("check_person_id(): is_id_match {}", is_id_match);

        return is_id_match && is_id_type_match;
    }
}
