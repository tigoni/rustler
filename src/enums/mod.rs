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

    //match Control
    //The match control is used to compare a value against a series of patterns and then execute code based on which pattern matches.
    //It is similar to a switch statement in other languages.
    //The match control is very powerful and can be used to destructure enums and structs.
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    pub fn render_item(direction: Direction) {
        match direction {
            Direction::Up => println!("Render up"),
            Direction::Down => println!("Render down"),
            Direction::Left => println!("Render left"),
            Direction::Right => println!("Render right"),
        }
    }

    //matching with Option<T>
    //The Option<T> enum is very commonly used in Rust code, so the match control is used to handle it.
    //The match control can be used to handle the Some and None variants of the Option<T> enum.
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    //using a catch-all pattern
    //The _ pattern can be used to match any value
    //It is used as a catch-all pattern to match any value that is not explicitly matched
    fn add_one_if_five(x: Option<i32>) -> Option<i32> {
        match x {
            Some(5) => Some(5 + 1),
            _ => None,
        }
    }

    //dice game with match control
    //The match control can be used to handle complex patterns
    //In this example, the match control is used to handle a dice game
    fn play() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }

    //a function that tests all the functions in this module
    pub fn test_enums_all() {
        test_enums();
        test_enum_with_data();
        test_enum_with_methods();
        test_option_enum();
        render_item(Direction::Up);
        render_item(Direction::Down);
        render_item(Direction::Left);
        render_item(Direction::Right);

        let five = Some(5);
        let result = plus_one(five);
        let res = add_one_if_five(five);
        println!("The result is {:?}", result);
    }
}
