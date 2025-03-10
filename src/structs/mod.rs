//Structs:
//they are similar to tuples, but they have names for their fields.
//Structs are useful for creating custom types that can be passed around in your code.
//Structs can be used to create complex data types that can be used to model real-world entities.
//Structs can be used to create data types that can be used to model real-world entities.

pub mod structs {

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    //builder function to create a user.
    //Because the email and username fields have the same name as the parameters, we can use the field init shorthand syntax to create a user from the parameters. This means we don’t have to specify the email and username fields in the user struct.
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    pub fn test_structs() {
        let user1 = User {
            email: String::from("James"),
            username: String::from("je@mail.com"),
            active: true,
            sign_in_count: 1,
        };
        //update the email
        //        user1.email = String::from("test@mail.com");//this will throw an error because the struct is immutable by default
        //to make the struct mutable, we use the mut keyword

        let mut user2 = User {
            email: String::from("James"),
            username: String::from("kism@mail.com"),
            active: true,
            sign_in_count: 1,
        };
        user2.email = String::from("tks@mail.com"); //works fine

        //another user can be created using the values of another user
        let user3 = User {
            active: user1.active,
            ..user1 //this is a struct update syntax
        };

        //user1 no longer available fully since its string values have been moved
        //to user3
        // println!("User1 email is {}", user1.email);

        //using tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
        //They are similatr to turples and ca be desrutctured and the '.' operator can be used to access the fields
        struct Color(i32, i32, i32);
        let black = Color(0, 0, 0);
        println!("The color is {} {} {}", black.0, black.1, black.2);
        //destructuring
        let Color(r, g, b) = black;
        println!("The color is {} {} {}", r, g, b);

        //unit-like structs
        //These are structs that don't have any fields
        //They are useful when you want to implement a trait on a type but don't have any data to store in the type itself.
        struct UnitLikeStruct;
        let u = UnitLikeStruct;
    }
}

pub mod rectangle {
    #[derive(Debug)]
    pub struct Rectangle {
        pub(crate) width: u32,
        pub(crate) height: u32,
    }

    //methods are defined within the context of a struct using the impl keyword
    //Everything in this block of code will be associated with the Rectangle struct./type
    impl Rectangle {
        //methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.
        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        //methods can also take another parameter
        //here the method takes another Rectangle instance and returns a boolean value if the current instance can hold the other instance (if the width and height of the current instance are greater than the width and height of the other instance)
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}
