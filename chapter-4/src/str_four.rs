fn main() {
    let mut owned_string: String = String::from("Rust");
    println!("Owned_string_one: {}", &owned_string);

    let borrowed_slice: &str = "Rust Book-01";
    let owned_slice_to_string: String = borrowed_slice.to_owned();
    println!("Borrowed-To-Owned: {owned_slice_to_string}");

    let mut pre_owned: &str = "Test Rust in details";
    let post_owned: String = pre_owned.to_owned();

    let mut mutable_str = post_owned;
    mutable_str.push_str(", with focus and analysis plan");
    println!("Mutable_String:{}", mutable_str);
}
