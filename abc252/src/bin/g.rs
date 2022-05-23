use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    // dp[i][j] means the number of ways to arrange the i-th to j-th elements as brothers.
    // Notice that dp[i + 1][j] represents the number of ways to make a new subtree using the i-th
    // element as the root. (So that the i+1-th to j-th elements are brothers).
    // Thus we set the initial value of dp to 1, because we need dp[i + 1][i] = 1.
    let mut dp = vec![vec![1usize; n]; n];

    for len in 2..=n {
        for l in 1..=n - len {
            let r = l + len - 1;
            dp[l][r] = dp[l + 1][r];
            for k in l + 1..=r {
                if p[l] < p[k] {
                    dp[l][r] += dp[l + 1][k - 1] * dp[k][r];
                    dp[l][r] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[1][n - 1]);
}
