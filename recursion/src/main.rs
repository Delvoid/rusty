fn main() {
    println!("Factorial of 5 is {}", fact(5));
}

fn fact(num: i32) -> i32 {
    if num == 1 {
        1
    } else {
        num * fact(num - 1)
    }
}
