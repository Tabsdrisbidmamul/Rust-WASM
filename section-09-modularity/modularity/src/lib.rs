pub trait Log {
    fn display_info(&self);
    fn alert(&self) {
        println!("ALERT")
    }
}

#[derive(Eq, PartialEq)]
pub enum PersonIdentity {
    Passport(String),
    IdentityCard(String),
}

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
    pub id: PersonIdentity,
}

pub struct Animal(pub String, pub u32, pub String);

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
    pub fn from(first_name: &str, last_name: &str, age: u32, id: PersonIdentity) -> Person {
        return Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            id,
        };
    }

    pub fn get_identity_value(&self) -> &String {
        let value = match &self.id {
            PersonIdentity::IdentityCard(s) => s,
            PersonIdentity::Passport(s) => s,
        };

        return value;
    }

    pub fn check_person_id(&self, to_match: &Person) -> bool {
        let self_id = self.get_identity_value();
        let to_match_id = to_match.get_identity_value();

        let is_id_type_match = self.id == to_match.id;

        let is_id_match = self_id == to_match_id;

        println!("check_person_id(): is_id_type_match {}", is_id_type_match);

        println!("check_person_id(): is_id_match {}", is_id_match);

        return is_id_match && is_id_type_match;
    }
}

pub fn log_info(value: impl Log) {
    value.alert();
}

pub fn log_info_dyn(value: &dyn Log) {
    value.alert()
}
