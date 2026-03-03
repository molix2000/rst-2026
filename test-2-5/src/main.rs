fn main() {
    println!("mutable and shadowing objects");
    let apples = 5;
    println!("Apples are {apples}");
    let mut apples = 55;
    println!("The mutable version of apples as shadowed are {apples}");

}
