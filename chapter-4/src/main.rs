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


 let pattern2 = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();

 let email = "example@domain.com";

 if let Some(captures) = pattern2.captures(email) {
  println!("User: {}", &captures[1]);
  println!("Domain: {}",&captures[2]);
  println!("TLD: {}", &captures[3]);

 }

 let pattern3 = Regex::new(r"\d+").unwrap();
 let pattern_target1 = "My phone num is 09765430212. ";
 let result1 = pattern3.replace_all(pattern_target1, "[REDACTED]");
 println!("Result1 is :{} " , result1);
}
