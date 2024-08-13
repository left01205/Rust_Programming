fn square(x: i32) -> i32
{
    let p=2;
    let mut t = 1;
    for _ in 1..=p 
    {
        t = t * x;
    }
    return t;

}
fn main() {
    let x = 5;
    let result = square(x);
    println!("Square of {} is {}", x, result);
}