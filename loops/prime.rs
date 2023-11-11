use std::env;
use std::str::FromStr;

fn sqrt_floor(n: i32) -> i32 {
    (n as f64).sqrt().floor() as i32
}

/// This program takes a number as an argument and checks if it is a prime number or not.
/// If the number is prime, it prints "{} is a prime", otherwise it prints "{} is not a prime number".
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a number as argument");
        return;
    }
    let n = i32::from_str(&args[1]).unwrap_or_else(|_| {
        println!("Please provide a valid number");
        std::process::exit(1);
    });
    let number = sqrt_floor(n);
    if n < 2 {
        println!("{} is a not prime number", n);
        return;
    }
    if n % 2 == 0 {
        println!("{} is not a prime number", n);
        return;
    }
    for i in (3..number).step_by(2) {
        println!("{}", i);
        if n % i == 0 {
            println!("{} is not prime number", n);
            return;
        }
    }
    println!("{} is a prime", n);
}
