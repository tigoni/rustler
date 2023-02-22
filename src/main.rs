


use crate::structures::testStruct::{User, Book, Point, Liquid};

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
            username,
            email,
        }
    }
    println!("Email is: {:?}", build_user(String::from("someemail.com"), String::from("anotherusername")));

    //using struct update syntax
    let mut book1 = Book {
        title: String::from("Guards Guards!"),
        author: String::from("Terry Pratchett"),
        price: 1250,
    };
    let mut book2 = Book {
        title: String::from("12 Rules for Life"),
        author: String::from("Jordan Peterson"),
        price: 1800,
    };

    //update book 2 using struct update syntax (like js spread syntax)
    let mut book3 = Book {
        title: String::from("Maps of Meaning"),
        ..book2 //this should be last 
    };

    println!("Book 3 description: {:?}", book3);

    //turple structs - No names associated with the fields, just types
    //Can be descructrured into individual pieces and the . operator can be 
    //used followed by the index tp accesss each value
    let point1 = Point(2,8);
    println!("{:?}", point1);
    println!("Y axix point = {:?}", point1.0);

    let liquid = Liquid;

    //using methods
    let discounted_price = book1.calc_discount();
    println!("The price of book1 after duscount: {discounted_price}");

}


