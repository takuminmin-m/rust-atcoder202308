use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut x_v_sum = vec![vec![0; w+1]; 1];
    let mut dot_v_sum = vec![vec![0; w+1]; 1];
    for x in 0..h {
        let mut x_n = vec![0; w+1];
        let mut dot_n = vec![0; w+1];
        for y in 1..w+1 {
            x_n[y] = x_n[y-1];
            dot_n[y] = dot_n[y-1];
            if s[x][y-1] == 'x' {
                x_n[y] += 1;
            } else if s[x][y-1] == '.' {
                dot_n[y] += 1;
            }
        }
        x_v_sum.push(x_n);
        dot_v_sum.push(dot_n);
    }

    let mut x_sum = vec![vec![0; w+1]; h+1];
    let mut dot_sum = vec![vec![0; w+1]; h+1];
    for y in 1..w+1 {
        for x in 1..h+1 {
            x_sum[x][y] = x_sum[x-1][y] + x_v_sum[x][y];
            dot_sum[x][y] = dot_sum[x-1][y] + dot_v_sum[x][y];
        }
    }
    // println!("{:?}", x_sum);

    let mut ans = 99999999;
    for x in 0..h {
        if w < k {
            break;
        }
        for y in 0..w-k+1 {
            let x_n = x_sum[x][y]+x_sum[x+1][y+k]-x_sum[x+1][y]-x_sum[x][y+k];
            if x_n == 0 {
                let dot_n = dot_sum[x][y]+dot_sum[x+1][y+k]-dot_sum[x+1][y]-dot_sum[x][y+k];
                ans = std::cmp::min(ans, dot_n);
                // println!("{} {} {} {}", x, y, dot_n, ans);
            }
        }
    }

    for y in 0..w {
        if h < k {
            break;
        }
        for x in 0..h-k+1 {
            let x_n = x_sum[x][y]+x_sum[x+k][y+1]-x_sum[x+k][y]-x_sum[x][y+1];
            if x_n == 0 {
                let dot_n = dot_sum[x][y]+dot_sum[x+k][y+1]-dot_sum[x+k][y]-dot_sum[x][y+1];
                ans = std::cmp::min(ans, dot_n);
            }
        }
    }

    println!("{:?}", if ans==99999999 { -1 } else { ans });
}
