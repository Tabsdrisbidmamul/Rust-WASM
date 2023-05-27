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
