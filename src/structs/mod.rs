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
    }
}
