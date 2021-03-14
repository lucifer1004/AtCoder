use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut dp = vec![1usize; k + 1];
    let mut pre = vec![0usize; k + 2];
    for ai in a {
        for i in 0..=k {
            pre[i + 1] = (pre[i] + dp[i]) % MOD;
        }
        for i in (0..=k).rev() {
            let lo = if i >= ai { i - ai } else { 0 };
            dp[i] = (pre[i + 1] - pre[lo] + MOD) % MOD;
        }
    }

    let mut ans = dp[k];
    if k >= 1 {
        ans = (ans - dp[k - 1] + MOD) % MOD;
    }
    println!("{}", ans);
}