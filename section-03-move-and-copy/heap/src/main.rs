fn main() {
    // let msg = String::from("Hello");
    let msg = "Hello";
    // let msg = "Hello";
    // let msg_2 = msg;

    // println!("{}", msg);

    print_message_add(msg);

    println!("{msg}");
}

fn print_message(msg: String) {
    println!("{msg}");

    let tempMsg = msg;
}

fn print_message_add(msg: &str) {
    println!("{msg}");

    let tempMsg = msg;
}
