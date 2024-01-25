use std::thread::JoinHandle;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut g = Vec::new();
    let mut seen = vec![false; n];

    for _ in 0..n {
        input! {
            a: i32,
        }
        g.push(a-1);
        if a != -1 {
            seen[a as usize - 1] = true;
        }
    }

    let last = seen.iter().position(|&x| !x).unwrap();
    let mut res = Vec::new();
    let mut i = last;
    loop {
        res.push(i+1);
        if g[i] < 0 {
            break;
        }
        i = g[i] as usize;
    }

    for j in 0..n-1 {
        print!("{} ", res[n-j-1]);
    }
    println!("{}", res.first().unwrap());
}
