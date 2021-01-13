use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        k: i32,
        h: [i32; n],
    }

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        for j in max(0, (i as i32) - k) as usize..i {
            dp[i] = min(dp[i], dp[j] + (h[i] - h[j]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
