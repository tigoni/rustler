
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
    println!("var1 = {var1}");


    //truples
    let turp: (i32, f32, i128) = (200, 28.23, 732832);
    let (x, y , z) = turp;
    println!("Turp is {x}, {y}, {z}");

    //arrays
    let a = [1,2,5,7];
    let b: [i32; 3] = [10,20,30];

    //initialise to contain 8 elements of the same value for each element 
    //same as let a = [3,3,3,3,3,3,3]
    let c = [3;8]; 

    //accessing
    let first = a[2]; //5
   
    //functions
    test_function(2425, 'H');
    let value = func_with_ret();
    println!("Value returned from func: {value}");

}

    fn test_function(x: i32, label: char) {
        println!("This is a test function with params {x} and {label}");
        let y = 5; //statement 
        // y * x //expression (no ending semicolons, adding one tunrs it into a statement and so no return value)
        println!("No return func called!")
    }

    fn func_with_ret() -> i32 {
        3 * 5
    }
 