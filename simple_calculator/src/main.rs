use std::io;
fn main() {
    loop {
        println!("Enter first number: ");
        let first_number = read_number();

        println!("Enter the operator (+, -, *, /): ");
        let operator = read_operator();

        println!("Enter second number: ");
        let second_number = read_number();

        let result = match operator {
            '+' => first_number + second_number,
            '-' => first_number - second_number,
            '*' => first_number * second_number,
            '/' => first_number / second_number,
            _ => panic!("Unknown operator"),
        };

        println!(
            "{} {} {} = {}",
            first_number, operator, second_number, result
        );

        println!("Do you want to continue? (y/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }
}

fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a number"),
        }
    }
}

fn read_operator() -> char {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let operator = input.trim().chars().next().unwrap_or_default();
        if operator == '+' || operator == '-' || operator == '*' || operator == '/' {
            return operator;
        } else {
            println!("Please enter an operator");
        }
    }
}
