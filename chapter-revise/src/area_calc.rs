pub fn area_calc() {
    println!("This is the area calculator segment");
    let width1 = 30;
    let height1 = 30;
    let rect1 = (30, 30);
    println!(
        "The area of the rectangle is {} pixels.",
        area(height1, width1)
    );
    println!("The area2 of the rectangle is {}", area2(rect1));
}

 pub fn area(height: u32, width: u32) -> u32 {
    height * width
}

pub fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
