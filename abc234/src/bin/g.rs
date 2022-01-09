use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp: Vec<usize> = vec![0; n];
    let mut hi: Vec<(usize, usize)> = vec![(a[0], 1)];
    let mut lo: Vec<(usize, usize)> = vec![(a[0], 1)];
    for i in 1..n {
        dp[i] = dp[i - 1];

        let mut acc: usize = dp[i - 1];
        while !hi.is_empty() && a[i] >= hi[hi.len() - 1].0 {
            dp[i] = (dp[i] + MOD - hi[hi.len() - 1].0 * hi[hi.len() - 1].1 % MOD) % MOD;
            acc = (acc + hi[hi.len() - 1].1) % MOD;
            hi.pop();
        }
        hi.push((a[i], acc));
        dp[i] = (dp[i] + a[i] * acc) % MOD;

        acc = dp[i - 1];
        while !lo.is_empty() && a[i] <= lo[lo.len() - 1].0 {
            dp[i] = (dp[i] + lo[lo.len() - 1].0 * lo[lo.len() - 1].1) % MOD;
            acc = (acc + lo[lo.len() - 1].1) % MOD;
            lo.pop();
        }
        lo.push((a[i], acc));
        dp[i] = (dp[i] + MOD - a[i] * acc % MOD) % MOD;
    }

    println!("{:?}", dp[n - 1]);
}
