use std::f32::consts::E;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut r = vec![100001; n];
    for i in 0..n {
        r[i] = if i==0 { 1 } else { r[i-1] };

        while r[i] < n && a[r[i]] - a[i] <= k {
            r[i] += 1;
        }
    }

    // println!("{:?}", r);

    let mut ans = 0;
    for i in 0..n {
        ans += r[i] - i-1;
    }
    println!("{}", ans);
}
