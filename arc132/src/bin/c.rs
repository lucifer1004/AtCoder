use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [i32; n],
    }

    let msk = 1 << (d * 2 + 1);
    let mut dp = vec![0usize; msk];
    dp[0] = 1;
    for (i, &ai) in a.iter().enumerate() {
        let mut ndp = vec![0usize; msk];

        for state in 0..msk {
            if dp[state] == 0 {
                continue;
            }

            let positions = if ai == -1 {
                (0..=d * 2).collect::<Vec<_>>()
            } else {
                vec![d + ai as usize - 1 - i]
            };

            for pos in positions {
                if state & (1 << pos) == 0 {
                    let mut nxt = state ^ (1 << pos);
                    if i >= d && (nxt & 1) == 0 {
                        continue;
                    }
                    nxt >>= 1;
                    ndp[nxt] = (ndp[nxt] + dp[state]) % MOD;
                }
            }
        }

        dp = ndp;
    }

    println!("{}", dp[(1 << d) - 1]);
}
