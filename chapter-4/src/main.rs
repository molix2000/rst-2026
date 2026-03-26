use regex::Regex;

fn main(){
   let pattern = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
   let date = "2026-09-12";

   if pattern.is_match(date) {
      println!("The date is the correct format. ");
   }
   else {
      println!("The Date is an incorrect format!");
   }

}
