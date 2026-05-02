struct user {

    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn main() {

 let user1 = user {
     active: true,
     username: String::from("samjit"),
     email: String::from("samjit@localhost.local"),
     sign_in_count: 1,
 }
; // Must Put this in



println!("The user name in user1 struct is {}", user1.username);
println!("The email for user1 is {}",user1.email);
println!("The sign_in_count for user1 is {}",user1.sign_in_count);


}