mod struct_one;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@gmail.com"),
        active: true,
        sign_in_count: 1,
        username: String::from("someone"),
    };

    let name = user1.username;
    user1.username = String::from("someone_else");

    //factory function to create instances
    let user2 = build_user(String::from("abc@gmai.com"), "abc".to_string());

    //Tuple Structs
    //useful when you want to give the tuple a name
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        //Field init shorthand syntax
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
