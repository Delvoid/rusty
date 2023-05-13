use std::vec;

struct User {
    username: String,
    _sign_in_count: u64,
    _active: bool,
}

struct Color(i32, i32, i32);

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, width: u32) {
        self.width = width;
    }
}
fn main() {
    let user1 = User {
        username: String::from("delvoid"),
        _active: true,
        _sign_in_count: 1,
    };

    println!("user1: {}", user1.username);

    let user2 = build_user(String::from("user2"));
    println!("user2: {}", user2.username);

    let _color = Color(255, 0, 0);

    let mut square = Square {
        width: 10,
        height: 10,
    };

    println!("square area: {}", square.area());
    println!("square width: {}", square.width());

    square.change_width(5);
    println!("square width: {} area: {}", square.width(), square.area());

    let mut car = Car {
        color: String::from("red"),
        mpg: 30,
        top_speed: 120,
    };

    let mut motorcycle = Motorcycle {
        color: String::from("green"),
        mpg: 80,
        top_speed: 210,
    };

    println!("{:?}", car);
    println!("{:?}", motorcycle);

    car.set_color(String::from("blue"));
    println!("{:?}", car);

    car.set_mpg(40);
    println!("{:?}", car);

    car.set_top_speed(140);
    println!("{:?}", car);

    trait_on_diff_types();

    print(vec![1, 2, 3]);
    print("Hello, world!");
    print(42);
    print(String::from("some string to test"));
}

fn build_user(username: String) -> User {
    User {
        username,
        _active: true,
        _sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Car {
    color: String,
    mpg: i8,
    top_speed: i16,
}

#[derive(Debug)]
struct Motorcycle {
    color: String,
    mpg: i8,
    top_speed: i16,
}

trait Properties {
    fn set_mpg(&mut self, mpg: i8);
    fn set_color(&mut self, color: String);
    fn set_top_speed(&mut self, top_speed: i16);
}

impl Properties for Car {
    fn set_mpg(&mut self, mpg: i8) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: i16) {
        self.top_speed = top_speed;
    }
}

impl Properties for Motorcycle {
    fn set_mpg(&mut self, mpg: i8) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: i16) {
        self.top_speed = top_speed;
    }
}

// ----------------
// Marker or Phantom Types
// ----------------

struct _Empty;
struct _Full;

struct _Channel<T, M> {
    message: Option<T>,
    state: M,
}
fn _marker_or_phantom() {
    // You can now create a channel in a particular state.
    let _empty_channel: _Channel<i32, _Empty> = _Channel {
        message: None,
        state: _Empty,
    };
    let _full_channel: _Channel<i32, _Full> = _Channel {
        message: Some(42),
        state: _Full,
    };
}

// ----------------
// Implementing trains on different types
// ----------------

trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn trait_on_diff_types() {
    // Usage
    let dog = Dog;
    let cat = Cat;
    dog.speak();
    cat.speak();
}

fn print<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}
