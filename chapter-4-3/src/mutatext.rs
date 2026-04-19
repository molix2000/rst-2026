pub fn mutatext() {
    let s = String::from("Rust own");
    println!("S {s}");

    let s2 = s;
    println!("s2 {s2}");
    // println!("s {s}"); This has now moved out of the scope, the value it once has is now assigned
    // to s2, only one object can own the value, its s2 that is owning the value of that string.
    //
    let sone = String::from("Rust own 2");
    let sone2 = &sone;
    println!("sone2 {sone2}");
    println!("sone {sone}");
    let sone3 = sone.clone();
    println!("sone3 {sone3}");
    println!("sone {sone}");
}
