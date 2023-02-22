pub mod testStruct {

#[derive(Debug)]
    pub struct User {
        pub active: bool,
        pub username: String,
        pub email: String,
    }


    #[derive(Debug)]
    pub struct Book {
        pub title: String,
        pub author: String,
        pub price: i32
    }

    //turple struct: No field names, just types.
    pub struct Color(pub i32, pub i32, pub i32);

    #[derive(Debug)]
    pub struct Point(pub i32, pub i32);

    //unit-like structs: No fields! 
    //useful for implementing traits on data types but no data to put in
    pub struct Liquid;


    //Method-Syntax: functions defined within the context of a struct
    //The first param is always self which represents the instance of the struct the 
    //function is being called on.

    //definition starts with an implementation block. Everything inside this block will be associated 
    //with Book type
    impl Book {
        pub fn calc_discount(&self) -> i32 {
            self.price * 2
        }

    }

}