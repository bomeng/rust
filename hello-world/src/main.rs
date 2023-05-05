fn main() {
    const SECOND_IN_MINUTE: u32 = 60;
    println!("{}", SECOND_IN_MINUTE);
    let x = 4;
    println!("x is: {}", x);
    {
       let x = 2;
       println!("x is: {}", x);
    }
    let x = x + 1;
    println!("x is: {}", x);
    println!("Hello, world!");
}
