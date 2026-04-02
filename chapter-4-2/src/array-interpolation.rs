use std::io;

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let mut index = String::new();
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Please input index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("There is an input error in data type");
    let index: usize = index.trim().parse().expect("This is an error in input");
    let element = months[index];
    println!("{}:", tup.0);
    println!("{}", months[0]);
    let element = months[index];
    println!("The value of the element at index {index} is : {element}");
}
