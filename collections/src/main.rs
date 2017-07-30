use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let v: Vec<i32> = Vec::new();
    println!("vector is {:?}", v);

    let v1 = vec![1, 2, 3];
    println!("vector is {:?}", v1);

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    println!("vector is {:?}", v2);

    let v3 = vec![1, 2, 3, 4, 5];
    let five1: &i32 = &v3[4];
    let five2: Option<&i32> = v3.get(4);
    println!("Results are {}, {:?}", five1, five2);

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("Six"))
    ];
    println!("Row is {:?}", row);

    let str = "initial data".to_string();
    println!("{}", str);

    let mut str1 = String::from("foo");
    str1.push_str("ba");
    str1.push('r');
    println!("Strint is {}", str1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);

    let len = String::from("Hola").len();
    let len1 = String::from("Здравствуйте").len();
    println!("Lengths are {}, {}", len, len1);

    let hello = String::from("Здравствуйте!");
    let answer = &hello[0..4];
    println!("{}", answer);

    for char in hello.chars() {
        println!("{}", char);
    }

    for byte in hello.bytes() {
        println!("{}", byte);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let score = vec![10, 50];
    let hash_score: HashMap<_, _> = teams.iter().zip(score.iter()).collect();
    println!("{:?}", hash_score);

    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

    let score = hash_score.get(&String::from("Blue"));

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }
    println!("{:?}", map);
}

enum VectorAttrs {
    Mean(f64),
    Median(i32),
    Mode(i32)
}

fn vector_attributes(vector: Vec<i32>) -> VectorAttrs {
    for number in vector.iter() {
        
    }
    vec!(0.1)
}
