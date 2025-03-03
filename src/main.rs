#![allow(unused_variables)]
//this is a shortcut to avoid repetition of the long path 
use crate::basics::variables::test_variables;
use crate::control::functions::test_functions;
use crate::ownership::ownership::test_ownership;
use crate::ownership::referenced::test_references;
use crate::ownership::slices::test_slices;
use crate::ownership::slices::first_word;
use crate::structs::structs::test_structs;
mod basics;
mod control;
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
test_structs();
}
