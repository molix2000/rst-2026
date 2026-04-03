


fn main(){

    println!("Funcmania begins");
    another_func_one();
    another_func_two(4);
    another_func_three(8,'R');
    let b = another_func_four(6);
   // link the output of function to the variable by returning the variable with no semicolong at the end of the line. 
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

fn another_func_four(input: i32) -> i32 {
    input // or any other i32 logic
}