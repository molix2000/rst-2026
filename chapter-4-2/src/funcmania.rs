


fn main(){

    println!("Funcmania begins");
    another_func_one();
    another_func_two(4);
    another_func_three(8,'R');
    let b = another_func_two(6);
    println!("{b}");
 } // end of main function
   //

fn another_func_one() {
   println!("Other_func_one has landed here");
}

fn another_func_two(x: i32){
   println!("The value x is: {x}");
}

fn another_func_three(value: i32, unit_label:char) {
    println!("The measurement is: {value},{unit_label}");


}

