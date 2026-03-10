use std::io;
fn main() {
    let mut x1 = String::from("MSX");
    let r1 = &mut x1;

    r1.insert(0, 'w');
    println!(" az is {}", x1);
}
