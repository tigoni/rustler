pub mod variables {

    pub fn test_variables() -> () {
        let mut x = 5;

        //un-signed (Can only contain positive numbers)
        const AMOUNT: u32 = 234;
        println!("Value of x: {x}");
        x = 12;
        println!("Value of x: {x}");
        println!("Value of AMOUNT: {AMOUNT}");

        //shadowing
        let var1 = "test";
        let var1 = var1.len();
        println!("var1 = {var1}");

        //Compound types: tuples and arrays
        //truples
        //Fixed length. Cannot grow or shrink
        let turp: (i32, f32, i128, u16) = (200, -28.23, 732832, 23);
        //Use destructuring to get values
        let (x, y, z, a) = turp;
        println!("Turp is {x}, {y}, {z} and {a}");
        
        //accessing an element
        let first = turp.0;
        let second = turp.1;
        println!("First element is {first} and second is {second}");

        //A tuple with one element is a different type from a tuple with two elements
        let one = (1,);
        let two = (1, 2);
        //The type of one is (i32,) and two is (i32, i32)
        //This is because the comma tells the compiler that one is a tuple with one element
        //and not just a number in brackets
        //This is called the single element tuple
        //It is useful when you want to have a tuple with one element
        //and not just a number in brackets.

        //A tuple with no elements is called an empty tuple
        //It is a type on its own and is written as ()
        //It is useful when you want to return a value from a function
        //but you don't have a value to return
        //It is also useful when you want to have a tuple with no elements
        //and not just an empty pair of brackets
        //It is called a unit and by default, it is the return type of functions
        //that don't return a value.

        //arrays
        //stores fixed number of elements of the same type in the stack
        //use when you know the number of elements you want to store
        //use when you want to allocate data on the stack rather than the heap
        //use when you want to store elements of the same type.
        let a = [1, 2, 5, 7];
        let b: [i32; 3] = [10, 20, 30];

        //initialise to contain 8 elements of the same value for each element
        //same as let a = [3,3,3,3,3,3,3]
        let c = [3; 8];

        //accessing
        let first = a[2]; //5

        let count: [u8;2] = [44,8]; 

        //vector
        //stores a variable number of elements of the same type in the heap
        //use when you don't know the number of elements you want to store
        //use when you want to allocate data on the heap.
    }
}
