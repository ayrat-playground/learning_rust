fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is '{}'", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Changes string: {}", s2);

    let first_word = first_word(&s2);
    println!("First word index is {}", first_word);

    let slice_first_word = first_word_with_slice(&s2);
    println!("First word is {}", slice_first_word);

    // s2.clear();


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
