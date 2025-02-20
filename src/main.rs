mod basics;
mod control;
mod ownership;
//this is a shortcut to avoid repetition of the long path 
use crate::basics::variables::test_variables;
use crate::control::functions::test_functions;
use crate::ownership::ownership::test_ownership;
fn main() {
 test_variables();
//  test_functions();
//  test_ownership();
}
