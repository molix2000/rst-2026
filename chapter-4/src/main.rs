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
}
