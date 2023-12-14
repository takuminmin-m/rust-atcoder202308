use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s_: Chars,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut res = if a<b {
        n*a
    } else {
        n*b
    };

    if n < 3 {
        println!("{}", res);
        return;
    }

    for i in 0..3 {
        let mut s = s_.clone();
        let mut p = 0;
        for _ in 0..i {
            p += a;
            s.remove(0);
        }
        while s.len() % 3 != 0 {
            p += b;
            s.pop();
        }

        let mut c_sum = vec![0; s.len()/3 + 1];
        for j in 0..s.len()/3 {
            let mut sub_sum = 0;
            for k in 0..3 {
                sub_sum += if k==0 {
                    if s[j*3+k]=='R' { 0 } else { c }
                } else if k==1{
                    if s[j*3+k]=='G' { 0 } else { c }
                } else {
                    if s[j*3+k]=='B' { 0 } else { c }
                }
            }

            c_sum[j+1] = c_sum[j] + sub_sum;
        }
        // println!("{:?}", s);
        // println!("{:?}", c_sum);

        let mut sub_res = std::usize::MAX;
        for l in 0..s.len()/3+1 {
            for r in 0..s.len()/3-l+1 {
                // println!("l{} r{}  {}", l, r, p + l*a*3 + r*b*3 + c_sum[s.len()/3-r] - c_sum[l]);
                sub_res = std::cmp::min(sub_res, p + l*a*3 + r*b*3 + c_sum[s.len()/3-r] - c_sum[l]);
            }
        }

        // println!("{}", sub_res);
        res = std::cmp::min(res, sub_res);
    }

    println!("{}", res);
}
