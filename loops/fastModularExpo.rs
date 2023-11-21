use std::env;
use std::str::FromStr;

fn fast_expo(mut a: i32, mut n: i32, mods: i32) -> i32 {
    let mut ans = 1;
    while n > 0 {
        if n % 2 == 0 {
            a = (a * a) % mods;
            n = n / 2;
        } else {
            ans = (ans * a) % mods;
            n -= 1;
        }
    }
    ans
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a number as argument");
        return;
    }
    let a = i32::from_str(&args[1]).unwrap_or_else(|_| {
        println!("Please provide a valid number");
        std::process::exit(1);
    });
    let n = i32::from_str(&args[2]).unwrap_or_else(|_| {
        println!("Please provide a valid number");
        std::process::exit(1);
    });
    let moda = i32::from_str(&args[3]).unwrap_or_else(|_| {
        println!("Please provide a valid number");
        std::process::exit(1);
    });
    println!("{}", fast_expo(a, n, moda));
}
