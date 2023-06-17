fn main() {
    let msg = String::from("Hello");
    let slice = &msg[2..4];

    let msg_clone = msg.clone();

    println!("{slice}\n{msg_clone}");

    let is_msg_equal = msg == msg_clone;

    println!("{}", is_msg_equal)
}
