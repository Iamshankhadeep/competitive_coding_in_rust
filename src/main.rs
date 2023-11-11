pub mod scanner {

    pub struct Scanner {
        buf: Vec<String>,
    }

    impl Scanner {
        pub fn new() -> Self {
            Self { buf: vec![] }
        }

        pub fn new_from(source: &str) -> Self {
            let source = String::from(source);
            let buf = Self::split(source);
            Self { buf }
        }

        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            loop {
                if let Some(x) = self.buf.pop() {
                    return x.parse().ok().expect("");
                }
                let mut source = String::new();
                std::io::stdin().read_line(&mut source).expect("");
                self.buf = Self::split(source);
            }
        }

        fn split(source: String) -> Vec<String> {
            source
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect::<Vec<_>>()
        }
    }
}

use crate::scanner::Scanner;
use std::{cmp::Reverse, collections::BinaryHeap, io::Write};
fn main() {
    let mut scanner = Scanner::new();
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let t: usize = scanner.next();
    for _ in 0..t {
        solve(&mut scanner, &mut out);
    }
}
fn solve(scanner: &mut Scanner, out: &mut std::io::BufWriter<std::io::StdoutLock>) {
    let n = scanner.next::<usize>();
    let m = scanner.next::<usize>();
    let a = (0..n).map(|_| scanner.next::<usize>()).collect::<Vec<_>>();
    let mut b = (0..m).map(|_| scanner.next::<usize>()).collect::<Vec<_>>();
    b.sort();
    b.reverse();
    let mut que = BinaryHeap::new();
    for i in 0..n {
        que.push((Reverse(a[i]), i));
    }
    let mut g = vec![vec![]; n];
    while let Some((Reverse(va), i)) = que.pop() {
        while let Some(&vb) = b.last() {
            if va < vb {
                break;
            }
            b.pop();
            g[i].push(vb);
        }
    }
    let mut ans = b;
    for i in 0..n {
        ans.push(a[i]);
        g[i].reverse();
        for &vb in &g[i] {
            ans.push(vb);
        }
    }
    for i in 0..ans.len() {
        if i > 0 {
            write!(out, " ").unwrap();
        }
        write!(out, "{}", ans[i]).unwrap();
    }
    writeln!(out, "").unwrap();
}
