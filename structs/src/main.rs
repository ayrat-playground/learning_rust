struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

struct AnotherRectangle {
    length: u32,
    width: u32
}

impl AnotherRectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, rectangle: &AnotherRectangle) -> bool {
        (self.length >= rectangle.length) && (self.width >= rectangle.width)
    }
}

fn main() {
    let user = build_user(String::from("email"), String::from("die_on_time"));
    println!("User username is {}", user.username);

    let new_user = change_username(user, String::from("die_tomorrow"));
    println!("User username is {}", new_user.username);

    let Color(a, b, c) = Color(5, 6, 7);
    println!("a is {}", a);

    let rectangle = Rectangle { width: 10, length: 20 };
    let area = area(&rectangle);
    println!("area is {}", area);
    println!("rectangle is {:#?}", rectangle);

    let another_rectangle = AnotherRectangle { width: 11, length: 21 };
    println!("area of anotherRectangle is {}", another_rectangle.area());

    let another_rectangle1 = AnotherRectangle { width: 9, length: 19 };
    println!("another_rectangle can hold? another_rectangle1: {}", another_rectangle.can_hold(&another_rectangle1));

    let square = Rectangle::square(10);
    println!("square is {:?}", square);
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

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
