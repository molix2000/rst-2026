fn main() {
    let owned = String::from("This is Rust");
    let str_slice: &str = owned.as_str();

    let str_slice_defer: &str = &*owned;

    let str_slice_defered_2: &str = &*owned;
    let str_slice_defered_3: &str = &*owned;

    println!("{str_slice_defer},{str_slice_defered_2},{str_slice_defered_3}");
} // end of main()
