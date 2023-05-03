fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x);

    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    let s1 = String::from("hello");

    let (s2, len) = calculate_length_takes_ownership(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calcualte_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn _let_owner() {
    let s1 = String::from("hello");
    let _s2 = s1; // The ownership is moved from s1 to s2.
                  // println!("{}", s1);  This will cause a compile-time error, as s1 is no longer the owner.

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // This will work, as i32 is a Copy type.
}

fn _ownership_scoped() {
    // s is not valid here, it’s not yet declared
    {
        let _s = "hello"; // s is valid from this point forward

        // do stuff with s
    }
    // this scope is now over, and s is no longer valid
}

fn _mutared_string() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

fn _move_ownership() {
    let s1 = String::from("hello");
    let _s2 = s1; // The ownership is moved from s1 to s2.
                  // println!("{}", s1);  This will cause a compile-time error, as s1 is no longer the owner.
}

fn _clone_ownership() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // The ownership is cloned from s1 to s2.

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length_takes_ownership(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calcualte_length(s: &String) -> usize {
    s.len()
}

fn _mutable_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// fn _dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn _first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
