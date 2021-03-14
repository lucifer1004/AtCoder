use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![0i64; n]; n];
    for i in 0..n {
        dp[i][i] = a[i];
    }
    for len in 2..=n {
        for l in 0..n - len + 1 {
            let r = l + len - 1;
            dp[l][r] = (a[l] - dp[l + 1][r]).max(a[r] - dp[l][r - 1]);
        }
    }
    println!("{}", dp[0][n - 1]);
}