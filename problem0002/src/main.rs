use std::io::{stdin, stdout, Write};
use std::process::exit;

fn read_limit_from_stdin() -> i32 {
    print!("Enter limit: ");
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
    let limit = read_limit_from_stdin();
    let mut a = 1;
    let mut b = 1;
    let mut c = 2;
    let mut sum = 0;

    while a < limit {
        sum += c;
        a = b + c;
        b = c + a;
        c = a + b;
    }

    println!(
        "The sum of the even-valued terms of the Fibonacci sequence: {}",
        sum
    );
}
