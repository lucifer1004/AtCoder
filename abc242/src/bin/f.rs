use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        w: usize,
    }

    let mut c = vec![vec![0usize; n * m + 1]; n * m + 1];
    for i in 1..=n * m {
        c[i][0] = 1;
        c[i][i] = 1;
        for j in 1..=i - 1 {
            c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) % MOD;
        }
    }

    let mut f = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if i * j < b {
                continue;
            }

            let mut ways = c[i * j][b];

            for p in 1..=i {
                for q in 1..=j {
                    if p == i && q == j {
                        continue;
                    }

                    let to_del = c[i][p] * c[j][q] % MOD * f[p][q] % MOD;
                    ways += MOD - to_del;
                }
            }

            f[i][j] = ways % MOD;
        }
    }

    let mut ans = 0;
    for i in 1..n {
        for j in 1..m {
            if i * j < b || (n - i) * (m - j) < w {
                continue;
            }

            ans += c[n][i] * c[m][j] % MOD * f[i][j] % MOD * c[(n - i) * (m - j)][w] % MOD;
            ans %= MOD;
        }
    }

    println!("{}", ans);
}
