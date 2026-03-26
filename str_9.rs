fn main() {
    let title: String = String::from("Rust Volumes");
    println!("Title:{}", title);

    let greeting = title.clone() + "This chapter is 5";
    println!("{greeting}");

    let full_list = format!("{}, {},", title,greeting);
    println!("{full_list}");
}
