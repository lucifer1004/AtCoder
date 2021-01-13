use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [[usize; 3]; n],
    }

    let mut dp = vec![0; 3];
    for i in 0..n {
        let mut ndp = vec![0; 3];
        for j in 0..3 {
            for k in 0..3 {
                if j != k {
                    ndp[k] = max(ndp[k], dp[j] + a[i][k]);
                }
            }
        }
        dp = ndp;
    }

    println!("{}", dp[0].max(max(dp[1], dp[2])));
}
