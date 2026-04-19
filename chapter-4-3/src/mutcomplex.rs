pub fn mutcomplex() {
    let s = String::from("Rust mutted");
    let r1 = s.clone();
    let r2 = s;

    println!("rs is {r1}, r2 is {r2}");
}
