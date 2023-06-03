fn main() {
    let mut message = String::from("Hello");
    message = extend_message(message);

    println!("{message}");
}

fn extend_message(mut msg: String) -> String {
    msg.push_str(" World");
    return msg;
}
