fn main()
{
    const PI: f32 = 3.14159;
    println!("Enter the radius of the circle");
    let r: f32 = { 
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().parse().expect("Invalid input")
    };
    let result = PI * r * r;    
    println!("Area of the circle with radius {} is {}",r, result);
    
}