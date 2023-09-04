use proconio::{ input, marker::Chars }; fn main() { input! { n: i32, m: i32, p: i32, } let mut ans = 0; for i in 1..n+1 { if (i-m)%p == 0 { ans += 1; } } println!("{}", ans); }
