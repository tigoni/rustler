use std::env::var;

fn main() {
    let mut x = 5;
    const AMOUNT:u32 = 234;
    println!("Value of x: {x}");
    x = 12;
    println!("Value of x: {x}");
    println!("Value of AMOUNT: {AMOUNT}");

    //shadowing
    let var1 = "test";
    let var1 = var1.len();
    println!("var1 = {var1}")
}
