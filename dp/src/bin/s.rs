use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        k: Chars,
        d: usize,
    }

    let mut dp = vec![vec![0usize; 2]; d];
    let c = k[0].to_string().parse::<usize>().unwrap();
    for i in 0..c {
        dp[i % d][0] += 1;
    }
    dp[c % d][1] = 1;
    for i in 1..k.len() {
        let c = k[i].to_string().parse::<usize>().unwrap();
        let mut ndp = vec![vec![0usize; 2]; d];
        for j in 0..d {
            for p in 0..10 {
                let nxt = (j + p) % d;
                if p < c {
                    ndp[nxt][0] += dp[j][0] + dp[j][1];
                    ndp[nxt][0] %= MOD;
                } else if p == c {
                    ndp[nxt][0] += dp[j][0];
                    ndp[nxt][0] %= MOD;
                    ndp[nxt][1] += dp[j][1];
                    ndp[nxt][1] %= MOD;
                } else {
                    ndp[nxt][0] += dp[j][0];
                    ndp[nxt][0] %= MOD;
                }
            }
        }
        dp = ndp;
    }

    println!("{}", (dp[0][0] + dp[0][1] - 1 + MOD) % MOD);
}