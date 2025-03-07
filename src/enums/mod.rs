// Enums are a way to define a type by enumerating its possible values.
// It allows a type to be defined as one of a set of possible variants.
// Enums are defined using the enum keyword.
// Enums can be used to create types that can only have one of a few values.
pub mod begin_enums {
    #[derive(Debug)]
    pub enum IpAddrKind {
        V4,
        V6,
    }


    //Define a function that takes an IpAddrKind enum as a parameter
    fn route(ip_kind: IpAddrKind) {
        println!("The kind of IP address is {:?}", ip_kind);
    }

    //Test the enum by creating instances of the enum variants and passing them to the route function
    pub fn test_enums() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        route(four);
        route(six);
    }

    // Enums can also have data associated with each variant. 
    // This data can be of any type, including other enums.
    #[derive(Debug)]
    pub enum IpAddr {
        V4(String),
        V6(String),
    }

    pub fn test_enum_with_data() {
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
    }

    //Enums can also have methods defined on them
    #[derive(Debug)]
    pub enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            //method body would be defined here
        }
    }

    //Test the enum with methods
    pub fn test_enum_with_methods() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    //The Option enum is a very useful enum that is defined in the standard library
    //It is used to represent a value that can be either something or nothing
    //It is defined as follows:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    //Example of using the Option enum
    pub fn test_option_enum() {
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;

        //using the some type in a calculation
        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        let sum = x + y.unwrap();
        println!("The sum is {}", sum);
    }
}

