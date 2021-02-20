use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
        m: u128,
    }

    let mut d: u128 = 0;
    let n = x.len();
    let x: Vec<u128> = x.iter().map(|c| c.to_string().parse::<u128>().unwrap()).collect();
    for i in 0..n {
        d = d.max(x[i]);
    }

    let mut lo = d + 1;
    let mut hi = m;

    if hi < d {
        println!("0");
        return;
    }

    if n == 1 {
        println!("1");
        return;
    }

    while lo <= hi {
        let mid = (lo + hi) >> 1;
        let mut base = 1;
        let mut sum = 0;
        let mut ok = true;
        for i in (0..n).rev() {
            sum += x[i] * base;
            if sum > m {
                ok = false;
                break;
            }
            base *= mid;
            if base > m && i != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("{}", hi - d);
}
