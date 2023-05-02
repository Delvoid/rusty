fn main() {
    let _x = 5;

    // default is f64
    let _x = 2.0;
    let _y: f32 = 3.0;

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // char type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    //------------------------
    // Compound Types
    //------------------------

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring
    let (_x, y, _z) = tup;

    println!("The value of _y is: {}", y);

    // access via index
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");

    // Array
    let _a = [1, 2, 3, 4, 5];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // explicit type annotation
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize array with same value
    let _a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

    // access via index
    let _first = _a[0];
    let _second = _a[1];

    // vectors
    let mut v = vec![1, 2, 3, 4, 5];
    let _v2: Vec<i32> = Vec::new();

    // updating a vector
    v.push(7);

    println!("The value of v is: {:?}", v);
    v.pop();
    v.remove(1);

    println!("The value of v is: {:?}", v);

    let vec: Vec<i32> = (0..5).collect();
    println!("The value of vec is: {:?}", vec);

    //iterate over vector
    for value in &v {
        println!("{}", value);
    }

    // slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("The value of slice is: {:?}", slice); // [2, 3]

    let _first_in_slice = slice[0];

    // iterate over slice
    for value in slice {
        println!("{}", value);
    }

    //string slices
    let string = String::from("Hello, world!");
    let hello_slice = &string[0..5];

    println!("The value of hello_slice is: {}", hello_slice);

    // strings
    let mut _s = String::new();
    let sting_from = String::from("initial contents");
    println!("The value of sting_from is: {}", sting_from);

    let to_string = "change to string".to_string();
    println!("The value of to_string is: {}", to_string);

    let name = "Delvoid".to_string();
    let new_name = name.replace("void", "");

    println!("The value of new_name is: {}", new_name);

    // &str = string slice or "stir"
    let str1 = "hello"; // &str
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("The value of str1 is: {}", str1);
    println!("The value of str2 is: {}", str2);
    println!("The value of str3 is: {}", str3);

    //compare string
    println!("{}", "ONE".to_lowercase() == "one");

    // string literals
    // useful when you dont want utf-8 encoding
    let rust = "\x52\x75\x73\x74";
    println!("The value of rust is: {}", rust);
}
