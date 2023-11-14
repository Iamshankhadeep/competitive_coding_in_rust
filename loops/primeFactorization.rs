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

    let mut vector = vec![0; n];

    let mut k = n;
    while k > 1 {
        for l in 2..n {
            while k % l == 0 {
                vector[l] = vector[l] + 1;
                k = k / l;
            }
        }
    }
    for i in 0..n {
        if vector[i] > 0 {
            println!("{} is {} times", i, vector[i]);
        }
    }
}
