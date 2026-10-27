fn main(){

    let numbs: (u32, f64,u8) = (500,6.4, 8);
    let one = numbs.0;
    let two = numbs.1;
    let three = numbs.2;
    println!("Numbs one is {one}, Numbs two is {two}, Numbs three is {three} ");


    let numos: [String; 5] = ["August".to_string(),"September".to_string(), "October".to_string(), "November".to_string(), "December".to_string()];

    println!("1st is {0}, 2nd is {1}, 3rd is {1}, 4th is {3}, 5th is {4}", numos[0],numos[1],numos[2],numos[3],numos[4]);
}