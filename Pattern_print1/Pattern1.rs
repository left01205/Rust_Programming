fn main() {
    let rows = 5; // Define the number of rows for the pattern

    for _ in 0..rows {
        for _ in 0..rows {
            print!("*");
        }
        println!("");
    }
}