fn power(x: i32,y:i32) -> i32
{
    let mut t = 1;
    for _ in 1..=y
    {
        t = t * x;
    }
    return t;

}
fn main() {
    println!("Enter the number to calculate power");
    let x: i32 = { 
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().parse().expect("Invalid input")
    };
    println!("Enter the power to calculate");
    let y: i32 = { 
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().parse().expect("Invalid input")
    };
    let result = power(x,y);
    println!("Resultant of power{} to {} is {}",y, x, result);
}