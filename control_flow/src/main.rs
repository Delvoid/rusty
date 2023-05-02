fn main() {
    let x = 5;
    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is greater than 5");
    }

    is_divisible_by();
    if_in_let();
    example_loop();
    loop_labels();
    while_loop();
    loop_collections();
    range_loop();
}

fn is_divisible_by() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 7 };
    println!("The value of number is: {}", number);
}

fn example_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{counter}");

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

fn loop_collections() {
    let v = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", v[index]);
        index += 1;
    }

    for element in v {
        println!("the value is: {}", element * 10);
    }
}

fn range_loop() {
    for number in (1..6).rev() {
        println!("{}!", number);
    }
}
