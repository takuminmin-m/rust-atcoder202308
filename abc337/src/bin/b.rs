use proconio::{ input, marker::Chars };

fn main() {
    input! {
        mut s: Chars,
    }

    s.dedup();
    let t = ['A', 'B', 'C', 'c'];
    let mut i = 0;

    for &c in s.iter() {
        while t[i] != c {
            if i == 3 { println!("No"); return; }
            i += 1;
        }
    }

    println!("Yes");
}
