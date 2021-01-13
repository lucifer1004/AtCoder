use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let mut a = 0;
    let mut b = (h[1] - h[0]).abs();

    for i in 2..n {
        let c = min(a + (h[i] - h[i - 2]).abs(), b + (h[i] - h[i - 1]).abs());
        a = b;
        b = c;
    }

    println!("{}", b);
}
