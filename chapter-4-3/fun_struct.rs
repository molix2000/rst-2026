struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,

}


pub fn main(){
    let user1_email = String::from("devops@localhost.local");
    let user1_username = String::from("devops");

   let user_1_struct = build_user(user1_email,user1_username);
   println!("The user_1_struct.sign_in_count, {}", user_1_struct.sign_in_count);  

}

pub fn build_user(email: String, username: String) -> User {
        User {
           active: true,
           username: username,
           email: email,
           sign_in_count: 1,
        }
}
