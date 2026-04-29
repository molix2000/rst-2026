

fn main(){

   struct User{
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
   }
  
   let user1 = User {
       active: true,
       username: String::from("samji234"),
       email: String::from("samji234@localhost.local"),
       sign_in_count: 1,
    };
   user1.email = String::from("2ndemail@localhost.local");
}

fn build_user (email: String, username: String) -> {
   User{
     active: true,
     username: username,
     email: email,
     sign_in_count: 1,
   }

}
