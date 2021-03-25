use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: [usize; n],
    };

    let mut dp = vec![0usize; 100 * n + 1];
    dp[50 * n] = 1;
    for i in 0..n {
        let mut ndp = dp.clone();
        if x[i] >= a {
            for j in 0..=100 * n - (x[i] - a) {
                ndp[j + x[i] - a] += dp[j];
            }
        } else {
            for j in a - x[i]..=100 * n {
                ndp[j + x[i] - a] += dp[j];
            }
        }
        dp = ndp;
    }

    println!("{}", dp[50 * n] - 1);
}
