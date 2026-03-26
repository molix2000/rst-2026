fn main(){

 let bytes: &[u8] = &[77,33,40,55];
 let str_frm_bytes = String::from_utf8(bytes.to_vec()).expect("err");

 println!("str_frm_bytes:{str_frm_bytes}");

 let num = 46;
 println!("{},Num",num);
 let string_from_num = num.to_string();
 println!("string_from_num: {string_from_num}");

 let rust: String = String::from("Rust");
 let road: String = "Road".to_string();
 let greeting = format!("{} {}", rust, road);
 println!("{greeting}");
 let greeting2 = "Rust beggining";

 let reverse:String = greeting2.chars().rev().collect();
 println!("{}",reverse);




}

