// fn main() {
//     let a = 10;
//     let b = a;
//     let c = 15;
//     let sum = sum(a, b);

//     println!("{}", sum);
// }

// fn sum(x: u32, y: u32) -> u32 {
//     return x + y;
// }

fn main() {
    c();
    d();
    f();
}

fn a() {
    println!("Calling a()");
    e();
}

fn b() {
    println!("Calling b()")
}

fn c() {
    println!("Calling c()")
}

fn d() {
    println!("Calling d()");
    a();
}

fn e() {
    println!("Calling e()");
}

fn f() {
    println!("Calling f()");
    b();
}
