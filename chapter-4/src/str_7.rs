fn main(){
    // the trick to solve errors was to use the word mut below.
    let mut stra: String = String::from("Test Rust");
    println!("stra: {stra}");
    let mut strb = stra.push_str(" Regulrarly, in cycles");
    //let mut strb:String  = stra;
    println!("strb:{strb}");
    let mut literal: &str = "Rust Chapter4";
    println!("Literal: {}",&literal.to_string());
}
