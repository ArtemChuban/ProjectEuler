use std::io::{stdin, stdout, Write};
use std::process::exit;

fn read_number_from_stdin() -> u64 {
    print!("Enter number: ");
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
    let target = match input.trim().parse::<u64>() {
        Ok(i) => i,
        Err(_) => {
            println!("Error: Can't parse input as unsigned integer number");
            exit(1);
        }
    };
    target
}

fn remove_factor(number: &mut u64, factor: u64, last_divisor: &mut u64, limit: &mut u64) {
    if *number % factor == 0 {
        *last_divisor = 2;
        *number /= factor;
        while *number % factor == 0 {
            *number /= factor;
        }
        *limit = (*number as f64).sqrt() as u64;
    }
}

fn main() {
    let mut number: u64 = read_number_from_stdin();
    let initial = number;
    let mut last_divisor: u64 = 1;
    let mut limit = (number as f64).sqrt() as u64;
    remove_factor(&mut number, 2, &mut last_divisor, &mut limit);
    let mut current: u64 = 3;
    while number > 1 && current <= limit {
        remove_factor(&mut number, current, &mut last_divisor, &mut limit);
        current += 2;
    }
    println!(
        "Largest prime factor of the number {}: {}",
        initial,
        if number == 1 { last_divisor } else { number }
    );
}
