use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a number as argument");
        return;
    }
    let n = usize::from_str(&args[1]).unwrap_or_else(|_| {
        println!("Please provide a valid number");
        std::process::exit(1);
    });
    let mut sieve = vec![true; n];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..n {
        if sieve[i] {
            let x = n / i + 1;
            for k in 2..x {
                if k * i < n {
                    sieve[k * i] = false;
                }
            }
        }
    }
    for (index, value) in sieve.iter().enumerate() {
        if *value {
            println!("{} is a prime number", index);
        }
    }
}
