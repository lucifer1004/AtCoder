use proconio::input;

const INF: usize = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![INF; n]; n];
    let mut pre = vec![0usize; n + 1];
    for i in 0..n {
        dp[i][i] = 0;
        pre[i + 1] = pre[i] + a[i];
    }

    for len in 2..=n {
        for l in 0..=n - len {
            let r = l + len - 1;
            let sum = pre[r + 1] - pre[l];
            for k in l..r {
                dp[l][r] = dp[l][r].min(dp[l][k] + dp[k + 1][r] + sum);
            }
        }
    }

    println!("{}", dp[0][n - 1]);
}

