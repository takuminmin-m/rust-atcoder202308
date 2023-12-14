use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut l = 0;
    let mut r = n-1;
    loop {
        let m = (l+r)/2;

        if a[m] == x {
            println!("{}", m+1);
            break;
        } else if a[m] > x {
            r = m-1;
        } else if a[m] < x {
            l = m+1;
        }
    }
}
