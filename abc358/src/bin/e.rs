use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        k: usize,
        c: [usize; 26],
    }

    let mut comb = vec![vec![0; k + 1]; k + 1];
    comb[0][0] = 1;
    for i in 1..=k {
        comb[i][0] = 1;
        for j in 1..=i {
            comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
        }
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for ci in c {
        let mut next = vec![0; k + 1];
        for i in 0..=k {
            if dp[i] == 0 {
                continue;
            }

            for j in 0..=ci.min(k - i) {
                next[i + j] = (next[i + j] + dp[i] * comb[i + j][j]) % MOD;
            }
        }
        dp = next;
    }

    let mut ans = 0;
    for i in 1..=k {
        ans = (ans + dp[i]) % MOD;
    }

    println!("{}", ans);
}
