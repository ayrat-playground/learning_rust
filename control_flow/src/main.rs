fn main() {
    let number = 5;

    if number < 6 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    if number % 2 == 0 {
        println!("the number is divisable by 2");
    } else if number % 3 == 0 {
        println!("the number is divisable by 3");
    } else if number % 5 == 0 {
        println!("the number is divisable by 5");
    }

    let new_number = if number == 5 {
        6
    } else {
        5
    };

    println!("the value of new_number is: {}", new_number)
}
