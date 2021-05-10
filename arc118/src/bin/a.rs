#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        k: usize,
    }

    println!("{}", k + (100 * k - 1) / t);
}
