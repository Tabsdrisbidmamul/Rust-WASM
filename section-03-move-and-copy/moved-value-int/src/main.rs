fn main() {
    let mut age = 30;
    age = extend_age(age);

    println!("{age}")
}

fn extend_age(mut value: u32) -> u32 {
    value += 100;
    return value;
}
