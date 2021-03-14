use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut sum = vec![vec![0usize; m]; n];
    for i in 1..m {
        sum[0][i] = 1;
    }
    for i in 1..n {
        for j in 1..m {
            sum[i][j] = sum[i - 1][j] * j % MOD;
        }
    }
    for i in 0..n {
        for j in 2..m {
            sum[i][j] = (sum[i][j] + sum[i][j - 1]) % MOD;
        }
    }

    let mut exp = vec![1usize; n + 1];
    for i in 1..=n {
        exp[i] = exp[i - 1] * m % MOD;
    }

    let mut ans = m;

    // Add a number in each turn
    for i in 2..=n {
        // Total cost when there are no 0 cost additions
        let tot = (exp[i] + ans * m) % MOD;

        // Total cost that can be saved
        // 1. XXX...XXXYY: m ^ (i - 1) ways
        let mut dec = exp[i - 1];

        // 2. XX...XXYZ...ZY (Z > Y, j Zs in total): m ^ (i - 2 - j) * (1 ^ j + ... + (m - 1) ^ j) ways
        for j in 1..i - 1 {
            dec = (dec + exp[i - 2 - j] * sum[j][m - 1]) % MOD;
        }
        ans = (tot + MOD - dec) % MOD;
    }

    println!("{}", ans);
}