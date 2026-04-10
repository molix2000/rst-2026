fn main() {

   let so = String::from("Rust String object");
   let leny = calculate_length(&so);

   println!("The length of the string is {leny}");


   /* ### extra borrowed checked example: ### */
   let mut data = vec![1,2,3];
   let ref = &data;
   data.push(4);
   println!("{:?}," ref);

}

fn calculate_length(s: String)-> usize {
    s.len()
}


