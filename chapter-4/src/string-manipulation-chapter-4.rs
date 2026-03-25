fn main() {
    let string_literal: &str = "Rust example";
    println!("{}", literal);

    let mut owned_string: String = String::from("Rust");
    owned_string.push_str(",world!");
    println!("{}", owned_string);
}
