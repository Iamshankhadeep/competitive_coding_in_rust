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
    let mut magical = n;
    while magical > 9 {
        let mut count = 0;
        let mut number = magical;
        while number != 0 {
            count = count + number % 10;
            number = number / 10;
        }
        magical = count;
    }
    println!("{}", magical);
}
