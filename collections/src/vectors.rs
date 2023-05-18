pub fn vectors() {
    let _v: Vec<i32> = Vec::new(); // an empty vector
    println!("Vec new() {:?}", _v);

    let _v = vec![1, 2, 3]; // a vector containing the values 1, 2, and 3
    println!("vec! macro {:?}", _v);

    let mut v: Vec<i32> = vec![];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8); // v is now [5, 6, 7, 8]

    println!("v {:?}", v);

    //we can also pop a value

    let pop = v.pop();
    println!("pop {:?}", pop);

    vector_read();
    vector_iterate()
}

fn vector_read() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn vector_iterate() {
    let v = vec![100, 69, 420];
    for i in &v {
        println!("{}", i);
    }

    // iterate over a mutable ref to each element
    // allowing you to modify the elements
    let mut v = vec![100, 32, 57];
    println!("modified vec before {:?}", v);

    for i in &mut v {
        *i += 50;
    } // v is now [150, 82, 107]

    println!("modified vec after {:?}", v);
}
