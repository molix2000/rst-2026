struct User<'a> {
    active: bool,
    username: &'a String,
    email: &'a String,
    sign_in_count: u64,
}

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let user3_name = String::from("example_user");
    let user3_email = String::from("example_user@localhost.local");
    let mut user1 = User {
        active: true,
        username: &String::from("samji234"),
        email: &String::from("samji234@localhost.local"),
        sign_in_count: 1,
    };
    user1.email = &String::from("2ndemail@localhost.local");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: &String::from("anoheruser@localhost.local"),
        sign_in_count: user1.sign_in_count,
    };
    let object = AlwaysEqual;
    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Colour is: {}, {}, {}", black.0, black.1, black.2);
    let user3 = build_user(&user3_email, &user3_name, 1);
    println!("User3 email: {}, username {}", user3.email, user3.username);
}

fn build_user(email: &str, username: &str, sign_in_count: u64) -> User {
    User {
        active: true,
        username: &username,
        email: email,
        sign_in_count: sign_in_count,
    }
}
