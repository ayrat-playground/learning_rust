fn main() {
    let emoji = '👾';
    println!("Emoji: {}", emoji);

    let tuple = (5, "rock", 6.0);
    let (x, _, z) = tuple;
    println!("Tuple: ({}, {}, {})", x, tuple.1, z);

    let array = [1, 2, 3, 4, 5];
    println!("Array: [{}, {}, {}]", array[0], array[1], array[2])
}
