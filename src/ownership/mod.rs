pub mod ownership {
    //ownership: A set of rules that determine how values are stored in memory.
    //1. Every value in Rust has an owner
    //2. There can only be one ownwer at a time
    //3. When the owner goes out of scope, the value is dropped.

    //braces determine the scope of a value and once the value moves out of the
    // scope, it is removed from memory.

    pub fn test_ownership() -> () {
        let value = gives_ownership(); //return value moved into value

        let s = String::from("hello");
        let mut s1 = String::from("hello");
        s1.push_str("Feb 2025");
        println!("{}", s1);
        //This creates a copy of the reference to s1. A reference contains the the name, poiner, length and capacity
        //of s1. After a copy is made the original reference becomes invalid. This is called a move.
        //This is done to prevent double free errors.
        let s2 = s1;
        //println!("s1 is {}", s1); //This will cause an error

        //clone: This creates a deep copy of the value of s2. This is useful when we want to create a new value
        //that is independent of the original value.
        //Unlike in a 'move', where only stack data is copied, in a clone, both stack and heap data are copied.
        let s3 = s2.clone();
        println!("s2 is {}", s2);
        println!("s3 is {}", s3);

        //ownership and functions
        let some_string = String::from("hello");
        takes_ownership(some_string); //some_string is moved into the function
                                      //This will cause an error because some_string has been moved into the function and is no longer valid. Drop has been called on it.
                                      // println!("{}", some_string); //This will cause an error

        //return values and scope
        let some_string = gives_ownership(); //the return value of gives_ownership is moved into some_string
        println!("{}", some_string);
    }

    //ownership and functions
    fn takes_ownership(some: String) {
        //'some' comes into scope
        println!("{}", some);
    } //'some' goes out of scope and drop is called. memory is freed

    //return values and scope
    //functions with return values give ownership of the return value to
    //the functions that call them
    fn gives_ownership() -> String {
        let some_string = String::from("some value");
        some_string // this return value will be moved to the function that called it.
    }
}

//references and borrowing
pub mod referenced {

    // A reference is an address to a value in memory. It can be followed to access the value without taking ownership of it.
    // References are immutable by default. This means that the value they point to cannot be changed.

    // References are created using the & symbol. The value of the reference is the memory address of the value it points to.
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    pub fn test_references() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); //passing a reference to s1
        println!("The length of '{}' is {}", s1, len); //s1 is still valid
    }
}
