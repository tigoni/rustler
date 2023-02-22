

use reader::reader::{simple_file_opener, file_opener_with_recover, using_unwrap_for_errors, using_expect_for_errors, read_username_from_file, read_username_from_file_shorted};
use std::io::{Read};

pub mod reader;

fn main() {

    //unrecoverable errors - calling panic  
    let vector = vec![2,6,8];
    // vector[4];
    // simple_file_opener();

    // file_opener_with_recover();

    // using_unwrap_for_errors();

    // using_expect_for_errors();
    // file_read_with_error_handling();

    let res = read_username_from_file_shorted();
    println!("Result: {:?}", res);

}

fn file_read_with_error_handling() {

    let mut username = String::new();

    let result= read_username_from_file();
    let contents = match result.unwrap().read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };
}


