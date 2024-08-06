use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter a number:");

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse()
        .expect("Please enter a valid number");

    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
