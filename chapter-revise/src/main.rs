mod area_calc;
use area_calc::area_calc;
use area_calc::area;
use area_calc::area2;
fn main() {
    let rect2 = (40, 40);
    
    println!("Revision segment!");
    println!("The area2 is {}", area2(rect2));
    println!(" The rectangle area is {}", area(33,33));
}
