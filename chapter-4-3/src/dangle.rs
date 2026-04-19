pub fn dangle() {
    let zind = String::from("Rust Test 01, before dangle, ");
    let reference_to_nothing = dangles(zind);
    println!("{reference_to_nothing}");
}

pub fn dangles(_s: String) -> String {
    let s = String::from("Rust domain new dangle");
    s
}
