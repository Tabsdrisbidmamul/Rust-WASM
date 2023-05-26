fn main() {
    let message: &str = "Hello World";
    let new_message = print_welcome(message);

    println!("new message {}", new_message)
}

fn print_welcome(msg: &str) -> &str {
    println!("{}", msg);

    return "hi there";
}
