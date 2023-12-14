use proconio::{ input, marker::Chars };
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut set = HashSet::new();
    for &e in a.iter() {
        set.insert(e);
    }

    for &e in a.iter() {
        if set.contains(&(e+3)) && set.contains(&(e+6)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
