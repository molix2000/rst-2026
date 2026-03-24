use std::fmt::format;

fn main() {
    println!("Chapter 4!");
    let mut s = String::from("This is chapter 4");
    s.push_str(" OF the Rust Book");
    println!("{s}");

    let mut stringer: &str = "There is a light";
    stringer = "That never Goes out";
    println!("{stringer}");
 
    let mut stringer2: &str = "There is a light ";
    let mut stringer3:&str = "That never goes out";
    //stringer2.push_str(stringer3); Kept as good reference to pitfalls
    //println!("{stringer2}");

    // One cannot add the two string either, so what to do?
    let stringer4 = format!("{}{}", stringer2,stringer3);
    println!("{stringer4}");

    let stringer5 = stringer2.to_owned() + stringer3;
    println!("{stringer5}");

    let mut z: char = 'a';
    // canno add a char to a &str, as below, but can add a char to a String
    // let stringer6: String = z.to_owned() + stringer2 + stringer3;
    // println!("{stringer6}");

    // Further bad assumptions and pitfalls.
    //let stringer7 : String = stringer2.to_owned() + z.to_string();
    //println!("{stringer7}");

    let alpha: &str = "start ";
    let beta: &str = "learning ";
    let gamma: &str = "Rust";
    let mut delta: String = String::new();
    delta.push_str(alpha);
    delta.push_str(beta);
    delta.push_str(gamma);
    println!("{delta}");
    let sigma = String::from(stringer2) + "Always at your sight";
    println!("{sigma}");
    let foo: String = "Rust Lang ".to_owned() + stringer3;
    println!("{foo}");
    let jetta: &str = "Volkeswagen Jetta";
    let mut car: String = jetta.to_string();
    car.push_str(" is a great car");
    println!("{car}");
    // More pitfalls highlighted below:
    // let volkeswagen: String =jetta.as_str().to_owned();
    // println!("{volkeswagen}");
    // let mut jc: String = String::from("Toyota") + alpha.as_str();
    // println!("{jc}");

    let owned_string = String::from("Rust By Example");
    let string_slice: &str = owned_string.as_str();
    println!("Owned_string: {owned_string}");
    println!("String_Slice: {string_slice}");
    let string_slice_deref: &str = &*owned_string;
    println!("String_Slice_Deref: {string_slice_deref}"); 
    let bytes: &[u8] = &[72,101,108,111];
    let string_from_bytes = String::from_utf8(bytes.to_vec()).expect("This could go wrong and did");
    println!("String from bytes: {string_from_bytes}");
    let num: i32 = 42;
    let string_from_num = num.to_string();
    println!("string from num:{string_from_num}");
    let age: String = "30".to_string();
    let name = "Sam";
    let emp_id  = format!("{},{}", name, age);
    println!("{emp_id}");
    let original = "Learn, Rust";
    let reversed: String = original.chars().rev().collect();
    println!("Originl: {original},Reversed: {reversed}");
    let gampa = "Trust Rust";
    let slice = &gampa[0..5];
    println!("Gampa: {gampa}");
    println!("Slice:{slice}");
    let text = " Rabbit jumps on to the bed";
    // if text.contains(Rabbit") {
       // println!("The text contains Rabbit");
   //} else {
        //println!("The test does not contain Rabbit");
}


// https://dev.to/alexmercedcoder/in-depth-guide-to-working-with-strings-in-rust-1522

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html