
use core::panic;
use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {

    //rust will call panic afeter we attempt to access an index beyond the vector size
    let vector = vec![2,6,8];
    // vector[93];

    //recoverable errors: The generic Result<T, E> is used.
    let result  = File::open("hello.txt");
    let file = match result {
       Ok(file) => file,
       Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}",other_error);
            }
       } 
    };

    //shortcuts for panic: unwrap and expect (both call panic if Ok is not returned)
    // let another_file = File::open("file1").unwrap();
    
    let yet_another_file = File::open("file2").expect("file 2 should be in this dir");

    //propagating errors

}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//Using the ? operator to propagate errors
fn read_again_from_file() -> Result<String, io::Error> {
    let mut username_file = file::open("")?; //? works almost inthe smae wayas the match expression
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username);
}