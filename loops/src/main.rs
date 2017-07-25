fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut number = 5;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    let a = [50, 40, 30, 20, 10];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is {}", element)
    }

    for range_number in (1..4).rev() {
        println!("{}!", range_number);
    }
}
