use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }

    let mut dp: Vec<f64> = vec![0.; n + 1];
    dp[0] = 1.;

    for i in 0..n {
        let mut ndp: Vec<f64> = vec![0.; n + 1];
        for j in 0..=i {
            ndp[j] += dp[j] * (1. - p[i]);
            ndp[j + 1] += dp[j] * p[i];
        }
        dp = ndp;
    }

    let mut ans = 0.;
    for i in (n + 1) / 2..=n {
        ans += dp[i];
    }

    println!("{}", ans);
}
