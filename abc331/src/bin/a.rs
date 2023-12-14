use proconio::{ input, marker::Chars };

fn main() {
    input! {
        yf: usize,
        mf: usize,
        y: usize,
        m: usize,
        d: usize,
    }

    let mut days = (y-1)*yf*mf + (m-1)*mf + d;
    // println!("{}", days);
    print!("{} ", days / (yf*mf) + 1);
    days %= yf*mf;
    print!("{} ", days / mf + 1);
    days %= mf;
    println!("{}", days + 1);
}
