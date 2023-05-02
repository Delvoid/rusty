fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(5);
    multiple_parameters(7, 8);
    expression();
    let rv = return_value();
    println!("The value of rv is: {}", rv);
    let po = plus_one(5);
    println!("The value of po is: {}", po);

    let higher_order = apply(double, 5);
    println!("The value of higher_order is: {}", higher_order);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let even_numbers = filter(&numbers, is_even);
    let odd_numbers = filter(&numbers, is_odd);

    println!("Even numbers: {:?}", even_numbers);
    println!("Odd numbers: {:?}", odd_numbers);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn multiple_parameters(x: i32, y: i32) {
    println!("x + y = {}", x + y);
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn return_value() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn public_function() {
    println!("I'm accessible from other modules!");
}

fn _private_function() {
    println!("I'm only accessible within the same module.");
}

// higher order functions
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

fn is_odd(n: &i32) -> bool {
    n % 2 != 0
}

fn filter<F>(items: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    let mut result = Vec::new();
    for item in items {
        if predicate(item) {
            result.push(*item);
        }
    }
    result
}
