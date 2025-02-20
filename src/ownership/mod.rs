pub mod ownership {
      //ownership
    //1. Every value in Rust has an owner
    //2. There can only be one ownwer at a time
    //3. When the owner goes out of scope, the value is dropped.

    //braces determine the scope of a value and once the value moves out of the 
    // scope, it is removed from memory.
      
    pub fn test_ownership() -> () {



    let value = gives_ownership(); //return value moved into value

    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("length of '{}' is {}", s, len);
    let mut s1 = String::from("hello");
    s1.push_str("hoew");
    println!("{}",s1);
    let s2 = s1; //this copies the pointer (string data location on heap),the lenght and capacity. It then invalidates s1 
    println!("s2 is {}", s2); //This will cause an error





    }
    //references and borrowing
    //references allow our code to use other variables without taking ownership
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
        //ownership and functions
        fn takes_ownership(some: String) { //'some' comes into scope
        println!("{}", some); 
    } //'some' goes out of scope and drop is called. memory is freed

    fn makes_copy(some_int: i32) { //some_int comes into scope
        println!("{}", some_int); 
    } //some_int goes out of scope. Nothing special happens

    //return values and scope
    //functions with return values give ownership of the return value to 
    //the functions that call them
    fn gives_ownership() -> String {
        let some_string = String::from("some value");
        some_string // this return value will be moved to the function that called it.
    }
}