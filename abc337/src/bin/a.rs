use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: [(usize, usize); n],
    }

    let mut a = 0;
    let mut b = 0;

    for &e in s.iter() {
        a += e.0;
        b += e.1;
    }

    print!("{}",
        if a > b {"Takahashi"} else if b > a {"Aoki"} else {"Draw"}
    )
}
