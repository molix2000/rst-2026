fn main() {
    let mut msga: &str = "Rust title";
    let mut msgb: String = msga.to_string();
    println!("{}", msgb);
    mainer();
}

fn mainer (){
   let owned_str = get_str();
   print_str(&owned_str);
}

fn get_str(){
   let s = String::from("Rust Chapter-04");
   // this below will return the value of s 
   s
}

fn print_str(my_str: &str){
   println!("{}", my_str);
}

