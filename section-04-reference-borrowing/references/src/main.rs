fn main() {
    let mut msg = String::from("Hello");
    let msg_ref = &mut msg;

    msg_ref.push_str(" World!");

    unpredictable_mutate(msg_ref);
    println!("original {msg}",);
    // println!("moved {msg_ref}");
}

fn unpredictable_mutate(value: &mut String) {
    value.push_str("_unpredictable");
}
