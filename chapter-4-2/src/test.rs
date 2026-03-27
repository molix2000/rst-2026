fn main() {
    let mut s = String::from("Rust");

    s.push_str(", Language!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`

    let s1 = String::from("Rust Lang ");
    let s2 = s1.clone();

    println!("{s1}, world!");

     // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
     //
     //
     let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
}
