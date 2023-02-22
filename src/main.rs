


use crate::structures::testStruct::User;

mod structures;
fn main() {

//create instance 
let mut user = User{
    active: true,
    username: String::from("goldenPlay"),
    email:String::from("gold.play@gd.ke")
    };

    user.email = String::from("another.emai.co");
    //a builder function to update the field
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
        }
    }
    println!("Email is: {:?}", build_user(String::from("someemail.com"), String::from("anotherusername")));

}


