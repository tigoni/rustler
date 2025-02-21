pub mod functions {
    /*Functions are declared using the fn keyword
    Functions can have parameters
    Functions can have return values
    Must have type annotations for parameters and return values
    The return value is the last expression in the function
    The return keyword is used to return early from a function
    The return type of a function is specified after an arrow ->
    The return type of a function is optional
    The return type of a function is inferred by the compiler.
    */

    /* Expressions vs Statements
    Expressions return a value
    Statements do not return a value
    Expressions do not include ending semicolons
    Statements include ending semicolons
    Expressions can be used as statements
    Statements cannot be used as expressions
    Adding a semicolon to an expression turns it into a statement.
     */
    pub fn simple_func() -> i32 {
        89 //this is an expression and returns a value 
    }


    pub fn test_functions() -> () {
        //functions
        println!("Simple func returns {}", simple_func());
        let value = func_with_ret();
        println!("Value returned from func: {value}");
        let result = comparator(23, 78);
        println!("Result of comparator: {result}");
        simple_check();
        // rep_with_loop();
        while_loop();
        loop_for_me();
    }

    fn func_without_ret(x: i32, label: char) {
        println!("This is a test function with params {x} and {label}");
        let y = 5; //statement
                   // y * x //expression (no ending semicolons, adding one tunrs it into a statement and so no return value)
        println!("No return func called!")
    }

    fn func_with_ret() -> i32 {
        3 * 5
    }

    //control flow
    fn comparator(param1: i32, param2: i32) -> i32 {
        if param1 > param2 {
            param1
        } else {
            param2
        }
    }

    //using if in a let statement
    fn simple_check() {
        //since if is an expression, it can be used in a let statement
        //the value of the let statement is the value of the block that is true
        let condition = false;
        let number = if condition { 5 } else { 9 };
        println!("number is: {number}");
    }

    //repetition
    pub fn rep_with_loop() {
        //Loop keyword is used to create an infinite loop
        //The break keyword is used to break out of a loop
        //The continue keyword is used to skip the rest of the loop and start the next iteration.
        let mut counter = 0;
        println!("Looping...");
        loop {
            counter += 1;
            print!("Counter is: {counter}");
            if counter == 10 {
                break;
            } else {
                println!("...not yet");
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
}
