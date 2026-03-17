fn main() {
    println!("Chapter-03-01!");
    let x = 5;
    let y = 10;
    println!("X is {x}, Y is {y}");
    let space_one = "     ";
    let space_one = space_one.len();
    println!("The space_one length is {space_one}");
    let guess : u32 = "42".parse().expect("there as an error");
    println!("Guess is {guess}");
    let sum = 5 + 10;
    println!("the sum is {sum}");
    let diff = 9.45 - 4.39;
    println!("The diff is {diff}");
    let product = 4 * 30;
    println!("product is {product}");
    let quoitent = 56.7 / 32.2;
    println!("the quotitent is {quoitent}");
    let remainder = 43 % 5;
    println!("The remainder is {remainder}");
    let t = true;
    let f: bool = false;
    println!("{t} and {f}");
    let tup: (i32, f64, u8) = (500,6.4,1);
    let (x,y,z) = tup;
    println!("{x}, {y}, {z}");
}
