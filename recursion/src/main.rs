fn main() {
    println!("Factorial of 5 is {}", fact(5));
    println!("Fibonacci of 5 is {}", fib(15));

    let string = "racecar";
    println!("{:?}", palindrome(string));

    let string = "hello";
    println!("{:?}", palindrome(string));
}

fn fact(num: i32) -> i32 {
    if num == 1 {
        1
    } else {
        num * fact(num - 1)
    }
}

fn fib(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    fib(num - 1) + fib(num - 2)
}

fn palindrome(word: &str) -> bool {
    if word.len() <= 1 {
        return true;
    }

    if word.chars().next() == word.chars().last() {
        palindrome(&word[1..word.len() - 1])
    } else {
        false
    }
}
