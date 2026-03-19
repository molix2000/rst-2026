fn main() {
    println!("Chapter 3.3!");
    println!(" Functions within main ");
    const A: i32 = 10;
    fn print_prompt_title (title: i32) {

        println!("this is the inner promprt printer function");
    }
    print_prompt_title(A);


    fn number_reflector (number: i32) {

        println!("The number is {number}");
    }
    number_reflector(A);
}