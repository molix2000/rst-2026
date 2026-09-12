fn area(dimentions: (u8, u8)) -> u8 {
    dimentions.0 * dimentions.1
}

fn ara_a(dems: u8, tems: u8) -> u8 {
    dems * tems
}

fn main() {
    let sector = (3, 4);
    let arc = 3;
    let arca = 22;
    println!("this is the area outcome {}", area(sector));
    println!("This is the ara_a resutl {}", ara_a(arc, arca));
}
