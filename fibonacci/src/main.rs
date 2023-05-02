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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(9), 34);
        assert_eq!(fibonacci(12), 144);
        assert_eq!(fibonacci(14), 377);
        assert_eq!(fibonacci(16), 987);
        assert_eq!(fibonacci(18), 2584);
        assert_eq!(fibonacci(23), 28657);
    }
}
