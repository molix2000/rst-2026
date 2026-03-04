fn main() {
    println!("mutable and shadowing objects");
    let apples = 5;
    println!("Apples are {apples}");
    let mut apples = 55;
    println!("The mutable version of apples as shadowed are {apples}");
    let x = 5;
    println!("The value of direct x is {x}");
    let x = 6;
    println!("The re-assigned value of x is {x}");
}
