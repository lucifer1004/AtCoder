use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let mut dp = vec![0usize; 1 << n];
    dp[0] = 1;
    for state in 0..(1 << n) - 1 {
        let cnt: usize = (state as usize).count_ones() as usize;
        for k in 0..n {
            if a[cnt][k] == 1 && (state & (1 << k)) == 0 {
                dp[state ^ (1 << k)] = (dp[state ^ (1 << k)] + dp[state]) % MOD;
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}