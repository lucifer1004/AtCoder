use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; m + 1]; m + 1]; m + 1];
    dp[m][m][m] = 1;

    for _ in 0..n {
        let mut ndp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; m + 1]; m + 1]; m + 1];

        for a in 0..=m {
            for b in a..=m {
                for c in b..=m {
                    if dp[a][b][c] == 0 {
                        continue;
                    }

                    let d = dp[a][b][c];
                    for i in 0..m.min(c + 1) {
                        if i == c || i == b || i == c {
                            ndp[a][b][c] = (ndp[a][b][c] + d) % MOD;
                        } else if i > b {
                            ndp[a][b][i] = (ndp[a][b][i] + d) % MOD;
                        } else if i > a {
                            ndp[a][i][c] = (ndp[a][i][c] + d) % MOD;
                        } else {
                            ndp[i][b][c] = (ndp[i][b][c] + d) % MOD;
                        }
                    }
                }
            }
        }

        dp = ndp;
    }

    let mut ans = 0;
    for i in 0..m {
        for j in i + 1..m {
            for k in j + 1..m {
                ans = (ans + dp[i][j][k]) % MOD;
            }
        }
    }

    println!("{}", ans);
}
