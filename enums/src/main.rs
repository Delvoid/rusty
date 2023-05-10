#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Method body here")
    }
}

// Message as structs
struct _QuitMessage; // unit struct
struct _MoveMessage {
    x: i32,
    y: i32,
}
struct _WriteMessage(String); // tuple struct
struct _ChangeColorMessage(i32, i32, i32); // tuple struct

enum Shape {
    Triangle,
    _Square,
    _Pentagon,
    _Octagon,
}

impl Shape {
    fn corners(&self) -> u8 {
        match self {
            Shape::Triangle => 3,
            Shape::_Square => 4,
            Shape::_Pentagon => 5,
            Shape::_Octagon => 8,
        }
    }
}
fn main() {
    let _home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", _home);

    route(IpAddrKind::V4(String::from("127.0.0.1")));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T> enum
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let sentence = "hello world";
    let word = find_word_starting_with(sentence, 'w');

    match word {
        Some(w) => println!("Found a word starting with w: {}", w),
        None => println!("No words starting with w"),
    }

    //match
    what_pet("dog");
    what_pet("cat");
    what_pet("bird");

    let x = 5;

    match x {
        1..=5 => println!("Matches"),
        _ => println!("Doesn't match"),
    }

    // if let

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //shape
    let triangle = Shape::Triangle;
    println!("A triangle has {} corners", triangle.corners());
}

fn route(_ip_kind: IpAddrKind) {}

fn find_word_starting_with(s: &str, letter: char) -> Option<&str> {
    s.split_whitespace().find(|&word| word.starts_with(letter))
}

fn what_pet(input: &str) {
    match input {
        "dog" => println!("I have a dog, woof!"),
        "cat" => println!("I have a cat, meow!"),
        _ => println!("I don't know what pet I have..."),
    }
}
