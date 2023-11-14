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
    if n <= 1 {
        println!("{}", n);
    }
    let mut vector = vec![0; n + 1];
    vector[1] = 1;
    for i in 2..n + 1 {
        vector[i] = vector[i - 1] + vector[i - 2];
    }
    println!("{}", vector[n]);
}
