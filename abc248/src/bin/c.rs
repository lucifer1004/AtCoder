use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![0; n * m + 1];
    dp[0] = 1;
    for i in 0..n {
        let mut ndp = vec![0; n * m + 1];
        for j in 0..=i * m {
            if dp[j] == 0 {
                continue;
            }
            for k in 1..=m {
                ndp[j + k] = (ndp[j + k] + dp[j]) % MOD;
            }
        }
        dp = ndp;
    }

    let mut ans = 0;
    for i in 1..=k {
        ans = (ans + dp[i]) % MOD;
    }

    println!("{}", ans);
}
