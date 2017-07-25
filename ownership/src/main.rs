fn main() {
    let str1 = String::from("rock");
    let str2 = str1.clone();

    println!("{} {}", str1, str2);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("world");

    let s3 = takes_and_gives_back(s2);

    let s4 = String::from("rock");

    let (s5, size) = calculate_length(s4);

    println!("The length of {} is {}", s5, size);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();

    (some_string, length)
}
