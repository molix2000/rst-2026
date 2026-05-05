pub fn main(){
    let user1_email = String::from("devops@localhost.local");
    let user1_username = String::from("devops");

   let user_1_struct = build_user(user1_email,user1_username);
   

}

pub fn build_user(email: String, username: String) -> User {
        User {
           active: true,
           username: username,
           email: email,
           signe_in_count: 1,
        };
}
