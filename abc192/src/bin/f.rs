use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }

    let mut ans = x;
    for i in 1..=n {
        let mut dp = vec![vec![-1; i]; i + 1];
        dp[0][0] = 0;
        for j in 0..n {
            let mut ndp = dp.clone();
            for k in 0..i {
                for t in 0..i {
                    if dp[k][t] == -1 {
                        continue;
                    }
                    let nxt = (dp[k][t] + a[j]) as usize % i;
                    ndp[k + 1][nxt] = ndp[k + 1][nxt].max(dp[k][t] + a[j]);
                }
            }
            dp = ndp;
        }
        if dp[i][x as usize % i] != -1 {
            ans = ans.min((x - dp[i][x as usize % i]) / i as i64);
        }
    }

    println!("{}", ans);
}
