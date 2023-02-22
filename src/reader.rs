pub mod reader {

    use std::fs::File;
    use std::io::{self, ErrorKind, Read};

    //recoverable errors:
    //A Result enum is used and contains generic Result<T, E>.
    //T is the type that will be returned in a succcessfull case within Ok
    //E is the Error type to be returned in the case of Failure

    //simple function to open an existing file. It fails if the function does not exist
    pub fn simple_file_opener() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Could not open the file: {:?}", error),
        };
    }
//Function to open an existing file, creates the file if it does not exist. Else it fails
    pub fn file_opener_with_recover() {
        let result = File::open("hello.txt");
        let file = match result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening file: {:?}", other_error);
                }
            },
        };
    }

    //using unwrap for error handling
    //unwrap will return the value on succcess and call panic if the Err type is returned.
    pub fn using_unwrap_for_errors () {
        let test_file = File::open("some_file").unwrap();

    }

    //expect is also the same but we can provide better messages
    pub fn using_expect_for_errors() {
        let target_file = File::open("my_file").expect("File cannot be found in this directory");
    }

    //propagating errors
    pub fn read_username_from_file() -> Result<File, io::Error> {

        let username_file_result = File::open("test.txt");
        return match username_file_result {
            Ok(file) => Ok(file),
            Err(e) => return Err(e),
        };
    }

    //propagating shortcut
     pub fn read_username_from_file_shorted() -> Result<String, io::Error> {

        let mut username_file = File::open("test.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

}
