fn main() {
    println!("In main.");

    function1();
    function2(10);
    function3(10, 55);
    function4();
}

fn function1() {
    println!("In function1.");
}

fn function2(value: i32) {
    println!("In function2. Value is {}.", value);
}

fn function3(value1: i32, value2: i32) {
    println!("In function3. Value1: {}, Value2: {}", value1, value2);
}

fn function4() {
    let value1 = {
        let value2 = 55;

        value2 + 5
    };

    println!("In function4. Value: {}", value1);
}
