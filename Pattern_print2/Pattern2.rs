fn main() {
    let height = 5;
    for i in 1..=height {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}