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

    for i in 1..n + 1 {
        for _j in 1..i + 1 {
            print!("*");
        }
        print!("\n");
    }
}
