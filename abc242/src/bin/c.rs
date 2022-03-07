use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![1usize; 10];
    for _ in 1..n {
        let mut ndp = dp.clone();
        for j in 1..=9 {
            if j + 1 <= 9 {
                ndp[j + 1] = (ndp[j + 1] + dp[j]) % MOD;
            }
            if j - 1 >= 1 {
                ndp[j - 1] = (ndp[j - 1] + dp[j]) % MOD;
            }
        }

        dp = ndp;
    }

    let mut ans = 0;
    for i in 1..=9 {
        ans += dp[i];
    }

    println!("{}", ans % MOD);
}
