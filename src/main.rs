

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
   
    //functionsb
    test_function(2425, 'H');
    let value = func_with_ret();
    println!("Value returned from func: {value}");
    let result = comparator(23, 78);
    println!("Result of comparator: {result}"); 
    simple_check();
    // rep_with_loop();
    while_loop();
    loop_for_me();

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

    //control flow
    fn comparator (param1: i32, param2: i32) -> i32 {
        if param1 > param2 {
            param1
        } else {
            param2
        }
    }

    //using if in a let statement 
    fn simple_check() {
        let condition = false;
        let number = if condition {5} else {9};
        println!("number is: {number}");
    } 

    //repetition
    fn rep_with_loop() {
        let mut counter = 0;
        loop {
            counter +=1;
            print!("again!");
            if counter == 10 {
                break;
            } else {
                continue;
            }
        }

        //a loop with a result
        let mut another_counter = 0;
        let output: i32 = loop {
            if counter <= 10 {
              counter += 1;
            //   break another_counter *= 50
            } else {
                continue;
            }
        };
        println!("Result is: {output}");
    }


    fn while_loop() {
        let mut number = 10;
        while number != 200 {
            print!("{number}, ");
            number += 10;
        }
        print!("{number}");
        println!();
    }

    //using for 
    fn loop_for_me() {
        for num in (1..23).rev() {
            print!("-{num}")
        }
        println!("Completed!")
    }