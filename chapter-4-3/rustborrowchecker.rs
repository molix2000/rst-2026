fn main() {
    // this should later be called multi_mut_example() //
    let mut x = 1;
    let mx1 = &mut x;
    //let mx2 = &mut x;
    // can't have a number of mutable references to the same object at the same time
    *mx1 = 2;
    // but can have multiple immutable references to the same object at the same time
    let mx2 = &x;
    println!("X = {}", mx1);
}
