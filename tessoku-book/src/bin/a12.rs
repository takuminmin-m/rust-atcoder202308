use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut l = 0;
    let mut r = 10_i32.pow(9) as usize;

    loop {
        let m = (l+r) / 2;
        let c = count(m, &a);
        if c < k {
            l = m+1;
        } else {
            r = m;
        }

        if l == r {
            break;
        }
    }

    println!("{}", r);
}

fn count(t: usize, a: &Vec<usize>) -> usize {
    let mut r = 0;
    for &e in a.iter() {
        r += t / e;
    }

    return r;
}
