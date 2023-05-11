extern crate rand;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut counter = 0;

    if args.len() < 2 {
        eprintln!("Please provide at least one target number.");
        std::process::exit(1);
    }

    let target_numbers: Vec<i32> = args[1..]
        .iter()
        .map(|x| {
            x.parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Failed to parse '{}'. Please provide integers.", x);
                std::process::exit(1);
            })
        })
        .collect();

    reroll(&mut counter, &target_numbers);
}

fn reroll(counter: &mut i32, target_numbers: &[i32]) {
    let mut rng = rand::thread_rng();

    loop {
        let roll = rng.gen_range(1..=6);
        *counter += 1;

        if target_numbers.contains(&roll) {
            println!("Rolled a {} after {} tries!", roll, *counter);
            break;
        }
    }
}
