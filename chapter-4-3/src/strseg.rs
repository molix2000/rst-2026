pub fn main() {
    let str1 = String::from("Rust str1");
    let str_literal = "Rust literal";

    let frstword = &str1[0..4];
    let secondword = &str1[5..9];
    let frstliteral = &str_literal[0..4];

    println!("First Word: {}", frstword);
    println!("Second Word: {}", secondword);
    println!("First Literal: {}", frstliteral);

    let worda = ownwrd(&str1[0..5]);
    println!("worda {worda}");
}
