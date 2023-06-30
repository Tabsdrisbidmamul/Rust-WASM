// use modularity::Animal;
// use modularity::Log;
// use modularity::Person;
// use modularity::PersonIdentity;

// use modularity::log_info;
// use modularity::log_info_dyn;

use modularity::{log_info, log_info_dyn, Animal, Log, Person, PersonIdentity};

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
