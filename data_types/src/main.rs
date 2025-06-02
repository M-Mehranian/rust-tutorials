use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Enter an array index.");
    
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Cannot read from input.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index is not a number.");

    let element = a[index];
    println!("The value of element at index {index} is {element}");
}