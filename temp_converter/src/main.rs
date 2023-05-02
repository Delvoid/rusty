use std::io;

fn main() {
    println!("Temperature Converter");

    let choice = loop {
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: u8 = match input.trim().parse() {
            Ok(num) => {
                if !(1..=2).contains(&num) {
                    println!("Invalid choice");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };

        break num;
    };

    let temperature = loop {
        println!("Enter temperature: ");

        let mut input_temperature = String::new();

        io::stdin()
            .read_line(&mut input_temperature)
            .expect("Failed to read line");

        let num: f32 = match input_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature");
                continue;
            }
        };

        break num;
    };

    match choice {
        1 => println!(
            "{}°F = {}°C",
            temperature,
            fahrenheit_to_celsius(temperature)
        ),
        2 => println!(
            "{}°C = {}°F",
            temperature,
            celsius_to_fahrenheit(temperature)
        ),
        _ => unreachable!(), // We've already validated the input, so this branch should never be reached.
    }
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}
