fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    // Safe way to handle potential out-of-bounds access using `get()`
    match numbers.get(index) {
        Some(number) => println!("The number at index {} is: {}", index, number),
        None => println!("Index {} is out of bounds", index),
    }

    // Alternative using `if let` for conciseness:
    if let Some(number) = numbers.get(index) {
        println!("The number at index {} is: {}", index, number);
    } else {
        println!("Index {} is out of bounds", index);
    }
} 