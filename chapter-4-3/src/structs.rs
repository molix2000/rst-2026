
struct User{
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
   }

struct Colour(i32,i32,i32);
struct Point(i32,i32,i32);
struct AlwaysEqual;

fn main(){
  
   let user1 = User {
       active: true,
       username: String::from("samji234"),
       email: String::from("samji234@localhost.local"),
       sign_in_count: 1,
    };
   user1.email = String::from("2ndemail@localhost.local");

   let user2 = User {
       active: user1.active,
       username: user1.username,
       email: String::from("anoheruser@localhost.local"),
       sign_in_count: user1.sign_in_count, 
   };
   let object = AlwaysEqual;
   let black = Colour(0,0,0);
   let origin = Point(0,0,0);
}

fn build_user (email: String, username: String) -> User {
   User{
     active: true,
     username: username,
     email: email,
     sign_in_count: 1,
   }

}
