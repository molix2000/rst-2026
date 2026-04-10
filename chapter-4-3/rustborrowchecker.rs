fn main() {
    // this should later be called multi_mut_example() //
    let mut x = 1;
    let mx1 = &mut x;
    //let mx2 = &mut x;

    *mx1 = 2;
    println!("X = {}", mx1);
}
