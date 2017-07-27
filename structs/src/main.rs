struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
struct Color(i32, i32, i32);

fn main() {
    let user = build_user(String::from("email"), String::from("die_on_time"));
    println!("User username is {}", user.username);

    let new_user = change_username(user, String::from("die_tomorrow"));
    println!("User username is {}", new_user.username);

    let Color(a, b, c) = Color(5, 6, 7);
    println!("a is {}", a);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn change_username(user: User, username: String) -> User {
    User {
        username,
        ..user
    }
}
