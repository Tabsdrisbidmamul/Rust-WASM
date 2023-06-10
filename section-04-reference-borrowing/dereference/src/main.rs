fn main() {
    let mut msg = String::from("Hello");
    let mut msg_ref = &mut msg;
    let msg_ref_ref = &mut msg_ref;

    msg_ref_ref.push_str(" World");

    println!("msg_ref_ref {}", msg_ref_ref);

    let first_value = 10;
    let second_value = &first_value;

    let is_value_equals = first_value == *second_value;

    println!("is_value_equals {}", is_value_equals);

    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;

    let e = &&100;
    c = e;

    println!(
        "&a: {:p} a: {} \n&b: {:p} b: {} \n&&c: {:p} c: {} \n&d: {:p} d: {} \n&&e: {:p} e: {}",
        &a, a, b, b, c, c, d, d, e, e
    )
}
