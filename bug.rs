fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    // This will cause a panic because index is out of bounds
    let number = numbers[index];
    println!("The number at index {} is: {}", index, number);
}