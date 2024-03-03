use std::io::{stdin, stdout, Write};
use std::process::exit;

fn sum_divisible_by(n: i32, target: i32) -> i32 {
    let p = (target - 1) / n;
    n * p * (p + 1) / 2
}

fn read_target_from_stdin() -> i32 {
    print!("Enter target number: ");
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => {
            println!("Error: Can't flush stdout");
            exit(1);
        }
    }
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => {
            println!("Error: Can't read from stdin");
            exit(1);
        }
    }
    let target = match input.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_) => {
            println!("Error: Can't parse input as integer number");
            exit(1);
        }
    };
    target
}

fn main() {
    let target = read_target_from_stdin();
    println!(
        "Sum of all the multiples of 3 or 5 below {}: {}",
        target,
        sum_divisible_by(3, target) + sum_divisible_by(5, target) - sum_divisible_by(15, target)
    );
}
