pub fn main() {
    let vec01 = vec!["Toyota", "Mitsubishi", "Mazda"];

    let values = display_vec_position_value(&vec01);
    println!("Returned values: {:?}", values);
  //  let elements = display_vec_elements_string(&vec01);
  //  println!("Returned elements: {:?}", elements);
}

pub fn display_vec_position_value<T: std::fmt::Display + Clone>(ve1: &[T]) -> Vec<String> {
    let mut values = Vec::new();

    for (position, value) in ve1.iter().enumerate() {
        println!("position {} = {}", position, value);
        values.push(value.to_string());
    }

    values
}

//pub fn display_vec_elements_string(vec01: &Vec<&str>) -> Vec<&str> {
//    for (i, &item) in vec01.iter().enumerate() {
//        println!("position {} = {}", i, item);
//    }
//    vec01.clone()  // Returns a copy of the vector with &str elements
//}