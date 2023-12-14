use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;
    for i in 0..s {
        for j in 0..n {
            if dp[j][i] {
                dp[j+1][i] = true;
                if i+a[j] > s {
                    continue;
                }
                dp[j+1][i+a[j]] = true;
            }
        }
    }

    println!("{:?}", dp);

    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
