use std::thread::JoinHandle;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        pa: [(usize, usize); n],
    }

    let mut p_sum = vec![vec![0; n+1]; m];
    for j in 0..m {
        for i in 0..n {
            p_sum[j][i+1] = p_sum[j][i];
            p_sum[j][i+1] += if pa[i].1 == j+1 {
                pa[i].0 / 2
            } else {
                pa[i].0
            };
        }

    }
    for _ in 0..q {
        input! {
            t: usize,
            l: usize,
            r: usize,
        }

        println!("{}" , p_sum[t-1][r]-p_sum[t-1][l-1]);
    }

    // println!("{:?}", p_sum);
}
