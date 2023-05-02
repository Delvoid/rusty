use std::io;

fn main() {
    println!("Nth Fibonacci number");

    let num = loop {
        println!("Enter a number: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };
        break num;
    };

    let result = fibonacci(num);
    println!("The {}th Fibonacci number is {}", num, result);
}

fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 2..=n {
        let tmp = a;
        a = b;
        b += tmp;
    }

    b
}
