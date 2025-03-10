#![allow(unused_variables)]
//this is a shortcut to avoid repetition of the long path
use crate::basics::variables::test_variables;
use crate::control::functions::test_functions;
use crate::enums::begin_enums::{
    test_enum_with_data, test_enum_with_methods, test_enums, test_option_enum,
};
use crate::ownership::ownership::test_ownership;
use crate::ownership::referenced::test_references;
use crate::ownership::slices::first_word;
use crate::ownership::slices::test_slices;
use crate::structs::rectangle::Rectangle;
use crate::structs::structs::test_structs;

mod basics;
mod control;
mod enums;
mod ownership;
mod structs;

fn main() {
    //  test_variables();
    //  test_functions();
    //  control::functions::rep_with_loop();
    //  test_ownership();
    //  test_references();
    // test_slices();
    // let s = String::from("hello world");
    // let word = first_word(&s);
    // println!("The first word is {}", word);
    // test_structs();
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    //area function
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!("The area of the rectangle is {}", area(&rect));

    println!("Rectangle is {:?}", rect);

    dbg!(&rect); //using debug macro as alternative to println! macro

    //using a method, we can define a method on a struct
    println!("The area of the rectangle is {}", rect.area());

    test_enums();
    test_enum_with_data();
    test_enum_with_methods();
    test_option_enum();
}
